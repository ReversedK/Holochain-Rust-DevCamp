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


/*********************************
 * ENTRY DEFINITION  
 * ************************* */

/***** STRUCTS DEFINITIONS ******/
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Profile {
    pub name: String,
    pub country: String,
}

/*********** VALIDATION DEFINITIONS ****************/
pub fn profile_definition() -> ValidatingEntryType {
    entry!(
        name: "profile",
        description: "A generic entry",
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
        utils.anchor("profile_directory"),
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
    from!("%agent_id", tag: "user_profile", 
        validation_package: || {hdk::ValidationPackageDefinition::ChainFull},
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| { Ok(()) }
        )
}



/* CHERCHER liste PROFILes */

// affiche array
pub fn handle_get_profile_list() -> JsonString {
    match get_profile_list() {
        Ok(result) => result.into(),
        Err(hdk_err) => hdk_err.into(),
    }
}

// Retrouve le lien vers le profil et le transmet ss forme de collection
fn get_profile_list() -> ZomeApiResult<Vec<Profile>> {
    utils::get_links_and_load(&DNA_HASH, "profile_directory").map(|results| {
        results
            .iter()
            .map(|get_links_result| {
                Profile::try_from(get_links_result.entry.value().clone()).unwrap()
            })
            .collect()
    })
}




/* RETROUVE UN LINK  ET SES OBJECTS*/

// affiche la collection
pub fn handle_get_my_profile() -> JsonString {    
    match get_link_to_a_profile(AGENT_ADDRESS.to_owned(),"user_profile".to_string()) {
        Ok(result) => result.into(),
        Err(hdk_err) => hdk_err.into(),
    }
}
// Retrouve le lien vers le profil et le transmet ss forme de collection
fn get_link_to_a_profile(address:Address,tag:String) -> ZomeApiResult<Vec<Profile>> {
    utils::get_links_and_load(&address, tag).map(|results| {
        results.iter().map(|get_links_result| {Profile::try_from(get_links_result.entry.value().clone()).unwrap()})
            .collect()
    })
}

/* LINKER */
fn linker(base:Address,address_entry:Address,tag:String) -> JsonString{
     match hdk::link_entries(&base,&address_entry,tag) {
                Ok(link_address) => address_entry.into(),
                Err(e) => e.into(),
            }
}
/* INSERER ENTRY ET LA LINKER */
pub fn handle_add_profile(name:String,country:String)->JsonString {
    let profile = Profile {name,country};
    let entry = Entry::new(EntryType::App("profile".into()), profile);
	   match hdk::commit_entry(&entry) {
        Ok(address) => {    
        linker(AGENT_ADDRESS.to_owned(),address.clone(),"user_profile".to_string()) ;          
        linker(DNA_HASH.to_owned().into(),address.clone(),"profile_directory".to_string()) 
      
        }
        Err(hdk_error) => hdk_error.into(),
    }   
}