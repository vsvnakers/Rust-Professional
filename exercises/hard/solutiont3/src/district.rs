use std::{collections::{HashMap, HashSet}, fs};

/// 并查集（Disjoint Set Union）结构体
struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    /// 创建一个新的并查集实例
    fn new() -> Self {
        DSU {
            parent: Vec::new(),
        }
    }

    /// 添加一个新的元素
    fn add(&mut self, n: usize) {
        self.parent.push(n);
    }

    /// 查找元素所属集合的代表元素（路径压缩优化）
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 路径压缩
        }
        self.parent[x]
    }

    /// 合并两个集合（按秩合并优化）
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // 按秩合并
            if root_x < root_y {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_x] = root_y;
            }
        }
    }

    /// 获取孤立部分的数量
    fn get_isolated_part(&mut self) -> usize {
        // 确保所有节点都指向其代表元素
        for i in 0..self.parent.len() {
            self.find(i);
        }
        // 使用 HashSet 计算唯一代表元素的数量
        let unique_elements: HashSet<_> = self.parent.iter().collect();
        unique_elements.len()
    }
}

/// 计算省份数量
///
/// # 返回值
/// 返回格式化的省份数量信息
pub fn count_provinces() -> String {
    // 读取 JSON 数据
    let json_data = fs::read_to_string("district.json").unwrap();
    let vec_input_str: Vec<&str> = json_data.lines().collect();
    let mut hashmap_input_data: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let mut is_first_marker = true;
    let mut cur_id = "0";
    let mut cur_data: Vec<String> = Vec::new();
    let mut cur_key = String::new();
    let mut cur_hashmap: HashMap<String, Vec<String>> = HashMap::new();

    // 解析输入数据
    for (i, line) in vec_input_str.iter().enumerate() {
        let line = line.trim();
        if line.contains("[") {
            if is_first_marker {
                // 获取当前省份 ID
                let temp: Vec<&str> = vec_input_str[i - 1].trim().split("\"").collect();
                cur_id = temp[1];
                cur_hashmap = HashMap::new();
                is_first_marker = false;
            }
            // 获取当前城市或地区的名称
            cur_key = line.split("\"").nth(1).unwrap().to_string();
            // 获取当前城市或地区的连接关系
            cur_data = line.split(",").map(|x| x.replace("\"", "")).collect();
            for item in cur_data.iter_mut() {
                if item.contains("[") {
                    *item = item.split("[").nth(1).unwrap().to_string();
                }
                if item.contains("]") {
                    *item = item.split("]").nth(0).unwrap().to_string();
                }
                *item = item.trim().to_string();
            }
            if cur_data.last().map_or(false, |s| s.is_empty()) {
                cur_data.pop();
            }
            // 更新当前省份的连接关系
            if let Some(existing_data) = cur_hashmap.get_mut(&cur_key) {
                existing_data.append(&mut cur_data);
            } else {
                cur_hashmap.insert(cur_key.clone(), cur_data.clone());
            }
        } else {
            if !is_first_marker {
                is_first_marker = true;
                hashmap_input_data.insert(cur_id.to_string(), cur_hashmap.clone());
            }
        }
    }

    let mut vec_ret = Vec::new();
    for (id, hashmap_case) in hashmap_input_data {
        let mut hashmap_k = HashMap::<String, usize>::new();
        let mut dsu = DSU::new();
        let mut cnt = 0;

        // 处理每个省份的城市和连接关系
        for (k, v) in hashmap_case {
            // 获取或创建城市节点
            let k_node = *hashmap_k.entry(k.clone()).or_insert_with(|| {
                dsu.add(cnt);
                cnt += 1;
                cnt - 1
            });

            for city in v {
                // 获取或创建连接城市节点
                let city_node = *hashmap_k.entry(city.clone()).or_insert_with(|| {
                    dsu.add(cnt);
                    cnt += 1;
                    cnt - 1
                });
                // 合并两个城市节点
                dsu.union(k_node, city_node);
            }
        }
        // 获取当前省份的孤立部分数量
        vec_ret.push((id, dsu.get_isolated_part()));
    }

    // 按省份 ID 排序结果
    vec_ret.sort_by(|x, y| x.0.cmp(&y.0));
    let mut string_ret = String::new();
    for item in vec_ret {
        string_ret.push_str(&item.1.to_string());
        string_ret.push_str(",");
    }
    // 去掉最后一个逗号并返回结果
    string_ret.trim_end_matches(",").to_string()
}