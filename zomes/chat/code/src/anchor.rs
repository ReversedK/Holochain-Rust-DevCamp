use hdk::{    
    holochain_core_types::{
        dna::zome::entry_types::Sharing,
        hash::HashString,
        json::JsonString,
        entry::Entry,
        entry::entry_type::EntryType,
        error::HolochainError,
        cas::content::Address,
    },

       self,
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS, DNA_HASH,debug
};
use serde_json::{Value, Error};

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
            name: "anchor",
            description: "A generic anchor",
            sharing: Sharing::Public,
            native_type: String,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |name: String, _ctx: hdk::ValidationData|{                 
                Ok(()) 
                },
            links : [
              profile_directory_link()
            ]              
        )
}

fn profile_directory_link() -> ValidatingLinkDefinition { 
    from!(
        "profile",
        tag: "profile_directory",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}


pub fn link_to_anchor(anchor_name:String,address:Address,tag:String)->JsonString{
    
    let anchor_entry = Entry::new(EntryType::App("anchor".into()), json!(anchor_name));
     match hdk::commit_entry(&anchor_entry){
        Ok(anchor_address)=>{
            match hdk::link_entries(&anchor_address,&address,tag) {
            Ok(())=>{
                hdk::debug("works!!!!!!!!!!!!");
                json!({"success":true}).into()
                },
            Err(err)=>{
                  hdk::debug(err);
                json!({"success":false,"err":"address not linked to anchor"}).into()
            }
            }
        }
        Err(err)=>json!({"success":false}).into()   
    }   


   // let anchor_instance = json::from_json_str(anchor_call).unwrap().into();
   // let anchor_address = anchor_instance.anchor_address;

}

pub fn getAddress(anchor_name:String)->Result<Address,()> {
     let anchor_entry = Entry::new(EntryType::App("anchor".into()), json!(anchor_name));   
     match hdk::commit_entry(&anchor_entry){
         Ok(addr)=>{            
             Ok(addr)
         },
         Err(err)=>{
              hdk::debug(err);
             Err(())
         }
     }
}