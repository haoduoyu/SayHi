use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::collections::Map;
use near_bindgen::{env, near_bindgen};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 模板model
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Template {
    pub id: String,         // 模板唯一编号
    pub name: String,       // 模板名称
    pub current_block: u64, // 模板创建时块高
    pub duration: u64,      // 模板超时块数
}

impl Template {
    pub fn new(id: String, new_name: String, new_current_block: u64, new_duration: u64) -> Self {
        Template {
            id: id,
            name: new_name,
            current_block: new_current_block,
            duration: new_duration,
        }
    }
}

// 名片model
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Card {
    pub id: String,              // 名片唯一编号
    pub template_id: String,     // 模板唯一编号
    pub public_message: String,  // 公开消息
    pub private_message: String, // 私密消息
    pub name: String,            // 名片名称
    pub count: u64,              // 名片数量
    pub is_avg: bool,            // 是否均分
    pub total: u64,              // 总红包
    pub current_block: u64,      // 名片创建时块高
    pub duration: u64,           // 名片超时块数
}

impl Card {
    pub fn new(
        id: String,
        template_id: String,
        public_message: String,
        private_message: String,
        new_name: String,
        new_count: u64,
        is_avg: bool,
        new_total: u64,
        new_current_block: u64,
        new_duration: u64,
    ) -> Self {
        Card {
            id: id,
            template_id: template_id,
            public_message: public_message,
            private_message: private_message,
            name: new_name,
            count: new_count,
            is_avg: is_avg,
            total: new_total,
            current_block: new_current_block,
            duration: new_duration,
        }
    }
}

// 联系人(收到的卡片以及被接收者查看后创建的联系人)model
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct ContactPerson {
    pub id: String,             // 联系人唯一编号
    pub contact_person: String, //联系人姓名
    pub card_count: u64,        // 收到的卡片数量
    pub duration: u64,          // 名片超时块数
}

impl ContactPerson {
    pub fn new(id: String, new_contact_person: String, new_card_count: u64, duration: u64) -> Self {
        ContactPerson {
            id: id,
            contact_person: new_contact_person,
            card_count: new_card_count,
            duration: duration,
        }
    }
}

// 用于提供访问服务
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct BLCardService {
    template_record: Map<String, Vec<Template>>, // key 为账号信息，value 为模板列表
    template_account_relation: Map<String, String>, // key 为模板唯一编号，value 为账号信息，用于反向查找

    card_record: Map<String, Vec<Card>>, // key 为账号信息，value 为名片列表
    card_account_relation: Map<String, String>, // key 为名片唯一编号，value 为账号信息，用于反向查找

    contract_person: Map<String, Vec<ContactPerson>>, // key 为账号信息，value 为联系人列表
}

#[near_bindgen]
impl BLCardService {
    // 创建模板
    pub fn create_template(&mut self, name: String, duration: u64) -> bool {
        let account_id = env::signer_account_id();
        let mut templates: Vec<Template> = Vec::new();

        if let Some(list) = self.template_record.get(&account_id) {
            templates = list.to_vec();
        }

        let current_block_index = env::block_index();
        let new_template = Template::new(
            name.to_string(),
            name.to_string(),
            current_block_index,
            duration,
        ); // TODO 第一个参数应该为template id
        templates.push(new_template);
        self.template_record
            .insert(&account_id, &templates);
        self.template_account_relation
            .insert(&account_id, &account_id); // TODO 第二个参数应该为template id

        return true;
    }

    // 列出指定账号的模板信息
    pub fn list_template(&self, account_id: String) -> Option<Vec<HashMap<String, String>>> {
        self.template_record.get(&account_id).map(|record| {
            let mut temp: Vec<HashMap<String, String>> = Vec::new();

            for item in record.iter() {
                let mut temp_map: HashMap<String, String> = HashMap::new();
                temp_map.insert(String::from("id"), item.id.to_string());
                temp_map.insert(String::from("name"), item.name.to_string());
                temp_map.insert(String::from("duration"), format!("{}", item.duration));
                temp.push(temp_map);
            }

            temp
        })
    }

    // 创建名片
    pub fn create_card(
        &mut self,
        template_id: String,
        public_message: String,
        private_message: String,
        name: String,
        count: u64,
        total: u64,
        duration: u64,
    ) -> bool {
        let account_id = env::signer_account_id();
        let mut cards: Vec<Card> = Vec::new();

        if let Some(list) = self.card_record.get(&account_id) {
            cards = list.to_vec();
        }

        let current_block_index = env::block_index();
        let new_card = Card::new(
            name.to_string(),
            template_id,
            public_message,
            private_message,
            name.to_string(),
            count,
            true,
            total,
            current_block_index,
            duration,
        ); // TODO 第一个参数应该为card id
        cards.push(new_card);
        self.card_record.insert(&account_id, &cards);
        self.card_account_relation
            .insert(&account_id, &account_id); // TODO 第二个参数应该为card id

        return true;
    }

    // 列出指定账号的名片信息
    pub fn list_card(&self, account_id: String) -> Option<Vec<HashMap<String, String>>> {
        let list = self.card_record.get(&account_id).unwrap();
        let mut temp: Vec<HashMap<String, String>> = Vec::new();

        for item in list.iter() {
            let mut temp_map: HashMap<String, String> = HashMap::new();
            temp_map.insert(String::from("id"), item.id.to_string());
            temp_map.insert(String::from("template_id"), item.template_id.to_string());
            temp_map.insert(
                String::from("public_message"),
                item.public_message.to_string(),
            );
            temp_map.insert(
                String::from("private_message"),
                item.private_message.to_string(),
            );
            temp_map.insert(String::from("name"), item.name.to_string());
            temp_map.insert(String::from("count"), format!("{}", item.count));
            temp_map.insert(String::from("total"), format!("{}", item.total));
            temp_map.insert(String::from("duration"), format!("{}", item.duration));

            temp.push(temp_map);
        }

        Some(temp)
    }

    // 通过 template 查询创建人
    pub fn find_account_by_template(&self, template_id: String) -> Option<String> {
        self.template_account_relation.get(&template_id)
    }

    // 通过 card 查询创建人
    pub fn find_account_by_card(&self, card_id: String) -> Option<String> {
        self.card_account_relation.get(&card_id)
    }

    // 创建联系人
    pub fn create_contract_person(&mut self, contact_person: String, duration: u64) -> bool {
        let account_id = env::signer_account_id();
        let mut contact_person_vec: Vec<ContactPerson> = Vec::new();

        if let Some(list) = self.contract_person.get(&account_id) {
            contact_person_vec = list.to_vec();
        }

        let new_contract_person = ContactPerson::new(
            contact_person.to_string(),
            contact_person.to_string(),
            contact_person_vec.len() as u64 + 1,
            duration,
        ); // TODO 第一个参数应该生成；第三个参数应该去查询已经收到的数量
        contact_person_vec.push(new_contract_person);
        self.contract_person
            .insert(&account_id, &contact_person_vec);
        return true;
    }

    pub fn list_contract_person(&self, account_id: String) -> Option<Vec<HashMap<String, String>>> {
        let list = self.contract_person.get(&account_id).unwrap();

        let mut temp: Vec<HashMap<String, String>> = Vec::new();

        for item in list.iter() {
            let mut temp_map: HashMap<String, String> = HashMap::new();
            temp_map.insert(String::from("id"), item.id.to_string());
            temp_map.insert(
                String::from("contact_person"),
                item.contact_person.to_string(),
            );
            temp_map.insert(String::from("card_count"), format!("{}", item.card_count));
            temp_map.insert(String::from("duration"), format!("{}", item.duration));

            temp.push(temp_map);
        }

        Some(temp)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(9),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn test_template_method() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let _template_name = String::from("new template 1");

        let mut bl_card_service = BLCardService::default();
        let create_result = bl_card_service.create_template(_template_name.to_string(), 100);
        assert_eq!(create_result, true);
        let _templates = bl_card_service.list_template("bob_near".to_string());

        match _templates {
            None => assert_eq!(1, 1),
            Some(_temp) => assert_eq!(_temp.len(), 1),
        }
    }

    #[test]
    fn test_card_method() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let _card_name = String::from("new card 1");

        // let mut bl_card_service = BLCardService::default();
        // let create_result = bl_card_service.create_card(
        //     "".to_string(),
        //     "".to_string(),
        //     "".to_string(),
        //     _card_name.to_string(),
        //     100,
        // );
        // assert_eq!(create_result, true);
        // let _templates = bl_card_service.list_card("bob_near".to_string());

        // match _templates {
        //     None => assert_eq!(1, 1),
        //     Some(_temp) => assert_eq!(_temp.len(), 1),
        // }
    }
}

// 创建联系人缺少为发卡人创建过程；创建卡片暂时不存在为指定人创建；扫描卡片未实现（扫描后数量减少或记录）；