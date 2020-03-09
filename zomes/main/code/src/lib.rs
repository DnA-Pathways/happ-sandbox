#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;
use if_chain::if_chain;
use std::convert::{TryFrom, TryInto};

// see https://developer.holochain.org/api/0.0.44-alpha3/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MyEntry {
    content: String,
    root: Option<Address>, // to place address of initial entry
}

impl TryFrom<Entry> for MyEntry {
    type Error = String;
    fn try_from(entry: Entry) -> Result<Self, String> {
        match entry {
            Entry::App(_entry_type, entry_value) => Ok(entry_value.try_into()?),
            _ => Err("Could not construct MyEntry from Entry".into()),
        }
    }
}

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn my_entry_def() -> ValidatingEntryType {
        entry!(
            name: "my_entry",
            description: "this is a same entry defintion",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
                Ok(())
            },
            links: [
                to!(
                    "my_entry",
                    link_type: "my_entry_to_my_entry",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[zome_fn("hc_public")]
    fn create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
        let create_entry = Entry::App("my_entry".into(), entry.clone().into());
        let root_address = hdk::commit_entry(&create_entry)?;

        let updated_address = hdk::update_entry(
            Entry::App(
                "my_entry".into(),
                MyEntry {
                    root: Some(root_address.clone()),
                    ..entry
                }
                .into(),
            ),
            &root_address,
        )?;

        Ok(updated_address)
    }

    #[zome_fn("hc_public")]
    fn update_my_entry(entry: MyEntry, address: Address) -> ZomeApiResult<Address> {
        // compare entries
        if let Some(old_entry) = hdk::get_entry(&address)? {
            let root_address = MyEntry::try_from(old_entry.clone())?.root.unwrap();

            let new_entry = Entry::App(
                "my_entry".into(),
                MyEntry {
                    root: Some(root_address),
                    ..entry
                }
                .into(),
            );

            if old_entry == new_entry {
                return Ok(address);
            }

            let updated_address = hdk::update_entry(new_entry, &address)?;

            Ok(updated_address)
        } else {
            Err(ZomeApiError::Internal("error".into()))
        }
    }

    #[zome_fn("hc_public")]
    fn get_my_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    #[zome_fn("hc_public")]
    fn link_my_entries(base_address: Address, target_address: Address) -> ZomeApiResult<Address> {
        if_chain! {
            if let Some(base_entry) = hdk::get_entry(&base_address)?;
            if let Some(target_entry) = hdk::get_entry(&target_address)?;
            then {
                let base_root = MyEntry::try_from(base_entry)?.root.unwrap();
                let target_root = MyEntry::try_from(target_entry)?.root.unwrap();
                hdk::link_entries(&base_root, &target_root, "my_entry_to_my_entry", "")
            } else {
                Err(ZomeApiError::Internal("error".into()))
            }
        }
    }

    #[zome_fn("hc_public")]
    fn get_linked_my_entries(base_address: Address) -> ZomeApiResult<GetLinksResult> {
        if let Some(base_entry) = hdk::get_entry(&base_address)? {
            let base_root = MyEntry::try_from(base_entry)?.root.unwrap();
            hdk::get_links(
                &base_root,
                LinkMatch::Exactly("my_entry_to_my_entry"),
                LinkMatch::Any,
            )
        } else {
            Err(ZomeApiError::Internal("error".into()))
        }
    }
}
