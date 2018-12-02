#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate machine_ip;

mod message;
mod channel;
mod profile;
mod utils;
mod anchor;


define_zome! {

	entries: [
		message::message_definition(),
    	channel::public_channel_definition(),
    	channel::direct_channel_definition(),
		profile::profile_definition(),
		anchor::entry_definition()
	]

    genesis: || {
        {
			Ok(())
        }
    }

	functions: {
		main (Public) {
			create_channel: {
				inputs: |name: String, description: String,  public: bool|,
				outputs: |result: JsonString|,
				handler: channel::handle_create_channel
			}
			get_my_channels: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: channel::handle_get_my_channels
			}
            post_message: {
				inputs: |channel_name: String, message: message::Message|,
				outputs: |result: JsonString|,
				handler: channel::handle_post_message
			}
			get_messages: {
				inputs: |channel_name: String|,
				outputs: |result: JsonString|,
				handler: channel::handle_get_messages
			}
			add_profile: {
				inputs: |name: String,country:String|,
				outputs: |result: JsonString|,
				handler: profile::handle_add_profile
			}
			get_my_profile: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: profile::handle_get_my_profile
			}
			get_profile_list: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: profile::handle_get_profile_list
			}
			
		
		}
	}
 }
