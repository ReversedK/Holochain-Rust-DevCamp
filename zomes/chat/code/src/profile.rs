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
use std::convert::TryFrom;
use super::utils;
use super::anchor;
use serde_json::{Value, Error};


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Profile {
    pub name: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Anchor {
    pub anchor_address: HashString  
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct StoreProfile {
    pub address: HashString ,
    pub entry:Profile
}


pub fn profile_definition() -> ValidatingEntryType {
    entry!(
        name: "profile",
        description: "A generic profile entry",
        sharing: Sharing::Public,
        native_type: Profile,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |profile: Profile, _ctx: hdk::ValidationData|{           
             match profile.name.len() >=2 {               
                false => Err(String::from("name too short")),
                true=>Ok(())                          
            }   
        },
        links : [agent_profile_link(),profile_directory_link()]     
    )
}

fn profile_directory_link() -> ValidatingLinkDefinition { 
    from!(
        "anchor",
        tag: "profile_directory",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn agent_profile_link() -> ValidatingLinkDefinition {
    from!(
        "%agent_id",
        tag: "user_profile",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}



/* CHERCHER liste PROFILes */

// affiche 
pub fn handle_get_profile_list() -> JsonString {    
    hdk::debug("getting profile list"); 
    match get_profile_list() {
        Ok(result) => result.into(),
        Err(hdk_err) => hdk_err.into(),
    }
}

// Retrouve le lien vers le profil et le transmet ss forme de collection
fn get_profile_list() -> ZomeApiResult<Vec<StoreProfile>> {
   let anchor_addr:HashString= anchor::getAddress("profile_directory".to_string()).clone().unwrap();
    utils::get_links_and_load(&anchor_addr, "profile_directory").map(|results| {
        results.iter().map(|get_links_result| {           
            StoreProfile {
                address : get_links_result.address.clone(),
                entry : Profile::try_from(get_links_result.entry.value().clone()).unwrap()
            }
                
            }).collect()
    })
}

/* RETROUVE UN LINK  ET SES OBJECTS*/
// affiche la collection
pub fn handle_get_my_profile() -> JsonString {    
    match get_link_to_my_profile(AGENT_ADDRESS.clone(),"user_profile".to_string()) {
        Ok(result) => result.into(),
        Err(hdk_err) => hdk_err.into(),
    }
}

// Retrouve le lien vers le profil et le transmet ss forme de collection
fn get_link_to_my_profile(address:Address,tag:String) -> ZomeApiResult<Vec<Profile>> {
    utils::get_links_and_load(&address, tag).map(|results| {
        results.iter().map(|get_links_result| {Profile::try_from(get_links_result.entry.value().clone()).unwrap()})
            .collect()
    })
}

pub fn handle_get_a_profile(addr:Address) -> JsonString {       
    match hdk::get_entry(addr.to_owned()) {
        Ok(result) => result.and_then(|entry| Some(entry.serialize())).into(),
        Err(hdk_err) => hdk_err.into(),
    }
}



/* Add profile, links it to the agent, then registers it to the profile_directory anchor */
pub fn handle_add_profile(name:String,country:String)->JsonString {
    hdk::debug("handlingProfileCreation");
    let profile = Profile {name,country};
    let entry = Entry::new(EntryType::App("profile".into()), profile);
	   match hdk::commit_entry(&entry) {
            Ok(address) => {         
            anchor::link_to_anchor("profile_directory".to_string(), address.clone(), "profile_directory".to_string());
            utils::linker(AGENT_ADDRESS.to_owned(),address.clone(),"user_profile".to_string()) 
        }
        Err(hdk_error) => hdk_error.into(),
    }   
}