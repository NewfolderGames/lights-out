use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{ closure::Closure, JsCast };
use web_sys::{ Document, Element, HtmlElement };
use crate::game::stuff::StuffManager;
use crate::utils::number::format_number_scientific;
use super::Tab;

struct UpgradePriceElement {

	pub root_element: Element,
	pub name_element: Element,
	pub count_element: Element

}

struct UpgradeElement {

	pub is_researched: bool,
	pub is_unlocked: bool,

	pub root_element: Element,
	pub title_element: Element,
	pub description_element: Element,
	pub price_container_element: Element,
	pub price_elements: HashMap<String, UpgradePriceElement>,

}

struct UpgradeCategoryElement {

	pub root_element: Element,
	pub button_element: Element,
	pub title_element: Element,
	pub list_element: Element

}


/// A upgrade tab.
pub struct UpgradeTab {

	tab_element: Element,
	tab_button_element: Element,

	locked_element: UpgradeCategoryElement,
	researchable_element: UpgradeCategoryElement,
	researched_element: UpgradeCategoryElement,
	upgrade_elements: HashMap<String, UpgradeElement>,

	is_selected: bool,
	is_unlocked: bool,

}

impl UpgradeTab {

	// Creates a new upgrade tab.
	pub fn new(document: Rc<Document>, stuff_manager: &StuffManager) -> Self {

		let tab_list_element = document.get_element_by_id("tab-list").expect("Element id 'tab-list' not found.");

		// Tab.

		let tab_element = document.get_element_by_id("tab-upgrade").unwrap();
		let tab_button_element = document.create_element("div").unwrap();

		tab_button_element.set_attribute("onclick", "Game.ui_change_tab('Upgrade')").unwrap();
		tab_button_element.set_inner_html(stuff_manager.get_text_string("ui_tab_upgrade").unwrap_or("TAB_TECHNOLOGY"));
		tab_button_element.set_class_name("button locked");

		tab_list_element.append_with_node_1(&tab_button_element).unwrap();

		// Categories.

		let locked_element = UpgradeCategoryElement {

			root_element: document.get_element_by_id("tab-upgrade-locked").unwrap(),
			button_element: document.create_element("button").unwrap(),
			title_element: document.create_element("div").unwrap(),
			list_element: document.create_element("ul").unwrap(),

		};
		let researchable_element = UpgradeCategoryElement {

			root_element: document.get_element_by_id("tab-upgrade-researchable").unwrap(),
			button_element: document.create_element("button").unwrap(),
			title_element: document.create_element("div").unwrap(),
			list_element: document.create_element("ul").unwrap(),

		};
		let researched_element = UpgradeCategoryElement {

			root_element: document.get_element_by_id("tab-upgrade-researched").unwrap(),
			button_element: document.create_element("button").unwrap(),
			title_element: document.create_element("div").unwrap(),
			list_element: document.create_element("ul").unwrap(),

		};

		// Set class name.

		locked_element.root_element.set_class_name("upgrade-category");
		locked_element.button_element.set_class_name("upgrade-category-button");
		locked_element.title_element.set_class_name("upgrade-category-title");
		locked_element.list_element.set_class_name("upgrade-category-list");

		researchable_element.root_element.set_class_name("upgrade-category");
		researchable_element.button_element.set_class_name("upgrade-category-button");
		researchable_element.title_element.set_class_name("upgrade-category-title");
		researchable_element.list_element.set_class_name("upgrade-category-list");

		researched_element.root_element.set_class_name("upgrade-category");
		researched_element.button_element.set_class_name("upgrade-category-button");
		researched_element.title_element.set_class_name("upgrade-category-title");
		researched_element.list_element.set_class_name("upgrade-category-list");

		// Append

		locked_element.root_element.append_with_node_1(&locked_element.button_element).unwrap();
		locked_element.root_element.append_with_node_1(&locked_element.title_element).unwrap();
		locked_element.root_element.append_with_node_1(&locked_element.list_element).unwrap();

		researchable_element.root_element.append_with_node_1(&researchable_element.button_element).unwrap();
		researchable_element.root_element.append_with_node_1(&researchable_element.title_element).unwrap();
		researchable_element.root_element.append_with_node_1(&researchable_element.list_element).unwrap();

		researched_element.root_element.append_with_node_1(&researched_element.button_element).unwrap();
		researched_element.root_element.append_with_node_1(&researched_element.title_element).unwrap();
		researched_element.root_element.append_with_node_1(&researched_element.list_element).unwrap();

		// Set inner html.

		locked_element.button_element.set_inner_html("Collapse");
		locked_element.title_element.set_inner_html("Locked technologies");

		researchable_element.button_element.set_inner_html("Collapse");
		researchable_element.title_element.set_inner_html("Researchable technologies");

		researched_element.button_element.set_inner_html("Collapse");
		researched_element.title_element.set_inner_html("Researched technologies");

		// Set click event.

		let closure_root_element = locked_element.root_element.clone();
		let closure_button_element = locked_element.button_element.clone();
		let closure = Closure::wrap(Box::new(move || {

			let root_element_class_list = closure_root_element.class_list();
			root_element_class_list.toggle("collapsed").unwrap();
			closure_button_element.set_inner_html(if root_element_class_list.contains("collapsed") { "Open" } else { "Collapse" });

		}) as Box<dyn Fn()>);
		locked_element.button_element.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(closure.as_ref().unchecked_ref()));
		closure.forget();

		let closure_root_element = researchable_element.root_element.clone();
		let closure_button_element = researchable_element.button_element.clone();
		let closure = Closure::wrap(Box::new(move || {

			let root_element_class_list = closure_root_element.class_list();
			root_element_class_list.toggle("collapsed").unwrap();
			closure_button_element.set_inner_html(if root_element_class_list.contains("collapsed") { "Open" } else { "Collapse" });

		}) as Box<dyn Fn()>);
		researchable_element.button_element.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(closure.as_ref().unchecked_ref()));
		closure.forget();

		let closure_root_element = researched_element.root_element.clone();
		let closure_button_element = researched_element.button_element.clone();
		let closure = Closure::wrap(Box::new(move || {

			let root_element_class_list = closure_root_element.class_list();
			root_element_class_list.toggle("collapsed").unwrap();
			closure_button_element.set_inner_html(if root_element_class_list.contains("collapsed") { "Open" } else { "Collapse" });

		}) as Box<dyn Fn()>);
		researched_element.button_element.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(closure.as_ref().unchecked_ref()));
		closure.forget();

		// Technologies.
		
		let mut upgrade_elements = HashMap::new();

		for (name, upgrade) in stuff_manager.iter_upgrade() {

			let mut upgrade_element = UpgradeElement {

				is_researched: upgrade.is_researched(),
				is_unlocked: upgrade.is_unlocked(),
				root_element: document.create_element("li").unwrap(),
				title_element: document.create_element("div").unwrap(),
				description_element: document.create_element("div").unwrap(),
				price_container_element: document.create_element("div").unwrap(),
				price_elements: HashMap::new()

			};

			upgrade_element.root_element.set_class_name("upgrade");
			upgrade_element.title_element.set_class_name("upgrade-title");
			upgrade_element.description_element.set_class_name("upgrade-description");
			upgrade_element.price_container_element.set_class_name("upgrade-price-container");

			upgrade_element.title_element.set_inner_html(stuff_manager.get_text_string(&format!("upgrade_{}_title", name)).unwrap_or(&format!("TECHNOLOGY_{}_TITLE", name.to_uppercase())));
			upgrade_element.description_element.set_inner_html(stuff_manager.get_text_string(&format!("upgrade_{}_description", name)).unwrap_or(&format!("TECHNOLOGY_{}_DESCRIPTION", name.to_uppercase())));

			upgrade_element.title_element.set_attribute("onclick", &format!("Game.purchase_upgrade('{}')", name)).unwrap();

			// Price.

			for (resource_name, resource_count) in upgrade.get_price() {

				let price_element = UpgradePriceElement {
					
					root_element: document.create_element("div").unwrap(),
					name_element: document.create_element("div").unwrap(),
					count_element: document.create_element("div").unwrap()

				};

				price_element.root_element.set_class_name("upgrade-price");
				price_element.name_element.set_class_name("upgrade-resource-name");
				price_element.count_element.set_class_name("upgrade-resource-count");

				price_element.name_element.set_inner_html(stuff_manager.get_text_string(&format!("resource_{}", resource_name)).unwrap_or(&format!("RESOURCE_{}", resource_name.to_uppercase())));
				price_element.count_element.set_inner_html(&format_number_scientific(*resource_count));

				upgrade_element.price_container_element.append_with_node_1(&price_element.root_element).unwrap();
				price_element.root_element.append_with_node_1(&price_element.name_element).unwrap();
				price_element.root_element.append_with_node_1(&price_element.count_element).unwrap();

				upgrade_element.price_elements.insert(String::from(resource_name), price_element);

			}
			
			// Append.

			if upgrade.is_researched() { researched_element.list_element.append_with_node_1(&upgrade_element.root_element).unwrap() }
			else if upgrade.is_unlocked() { researchable_element.list_element.append_with_node_1(&upgrade_element.root_element).unwrap() }
			else { locked_element.list_element.append_with_node_1(&upgrade_element.root_element).unwrap() }

			upgrade_element.root_element.append_with_node_1(&upgrade_element.title_element).unwrap();
			upgrade_element.root_element.append_with_node_1(&upgrade_element.description_element).unwrap();
			upgrade_element.root_element.append_with_node_1(&upgrade_element.price_container_element).unwrap();

			upgrade_elements.insert(String::from(name), upgrade_element);

		}

		Self {

			tab_element,
			tab_button_element,
			locked_element,
			researchable_element,
			researched_element,
			upgrade_elements,
			is_selected: false,
			is_unlocked: false,

		}

	}

}

impl Tab for UpgradeTab {

	fn is_selected(&self) -> bool {
		
		self.is_selected

	}

	fn render(&mut self, stuff_manager: &StuffManager) {

		// Tab.

		if stuff_manager.is_feature_unlocked("tab_upgrade") && !self.is_unlocked {

			self.is_unlocked = true;
			self.tab_element.class_list().remove_1("locked").unwrap();
			self.tab_button_element.class_list().remove_1("locked").unwrap();

		}

		if self.is_selected {
			
			self.tab_element.class_list().add_1("active").unwrap();
			self.tab_button_element.class_list().add_1("active").unwrap();

		} else {

			self.tab_element.class_list().remove_1("active").unwrap();
			self.tab_button_element.class_list().remove_1("active").unwrap();
			return

		}

		// Technologies.

		for (name, upgrade) in stuff_manager.iter_upgrade() {

			let upgrade_element = self.upgrade_elements.get_mut(name).unwrap();

			// Move upgrade.

			if upgrade.is_unlocked() && !upgrade_element.is_unlocked {

				upgrade_element.is_unlocked = true;
				self.researchable_element.list_element.append_with_node_1(&upgrade_element.root_element).unwrap();

			}

			if upgrade.is_researched() && !upgrade_element.is_researched {

				upgrade_element.is_researched = true;
				self.researched_element.list_element.append_with_node_1(&upgrade_element.root_element).unwrap();

			}

			if !upgrade_element.is_unlocked{ continue; }
			
			// Update price.

			for (resource_name, resource_count) in upgrade.get_price() {

				let price_element = upgrade_element.price_elements.get(resource_name).unwrap();
				let current_resource_count = stuff_manager.get_resource(resource_name).unwrap().get_count();

				if current_resource_count >= *resource_count {

					price_element.count_element.set_class_name("upgrade-resuorce-count");
					price_element.count_element.set_inner_html(&format_number_scientific(*resource_count));

				} else {

					price_element.count_element.set_class_name("upgrade-resource-count needs_more");
					price_element.count_element.set_inner_html(&format!("{} / {}", format_number_scientific(current_resource_count), format_number_scientific(*resource_count)));

				}

			}

		}

	}

	fn set_selected(&mut self, selected: bool) {
		
		self.is_selected = selected;

	}

}
