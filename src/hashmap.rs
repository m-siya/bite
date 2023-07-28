const TABLE_MAX_LOAD: f64 = 0.75;

#[derive(Clone)]
struct ObjString {
    length: i32,
    chars: String,
    hash: u32,
}

impl ObjString {
    fn new(chars: &str, length: i32, hash: u32) -> Self {
        Self {
            length,
            chars: chars.to_string(),
            hash,
        }
    }
}

struct Entry {
    key: Option<ObjString>,
    value: Value,
}

impl Entry {
    fn new() -> Self {
        Self {
            key: None,
            value: Value::Nil,
        }
    }
}

struct Table {
    count: i32,
    capacity: i32,
    entries: Vec<Entry>,
}

impl Table {
    fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            entries: Vec::new(),
        }
    }

    fn init_table(&mut self) {
        self.count = 0;
        self.capacity = 0;
        self.entries = Vec::new();
    }

    fn adjust_capacity(&mut self, capacity: i32) {
        let mut entries = vec![Entry::new(); capacity as usize];
        for i in 0..capacity as usize {
            entries[i].key = None;
            entries[i].value = Value::Nil;
        }
        for i in 0..self.capacity as usize {
            let entry = &self.entries[i];
            if entry.key.is_none() {
                continue;
            }
            let index = Self::find_entry_index(&entries, capacity, &entry.key.as_ref().unwrap());
            entries[index].key = entry.key.clone();
            entries[index].value = entry.value;
        }
        self.entries = entries;
        self.capacity = capacity;
    }

    fn find_entry_index(entries: &[Entry], capacity: i32, key: &ObjString) -> usize {
        let mut index = key.hash as usize % capacity as usize;
        loop {
            let entry = &entries[index];
            if entry.key.is_none() || entry.key.as_ref().unwrap().eq(key) {
                return index;
            }
            index = (index + 1) % capacity as usize;
        }
    }

    fn find_entry(&self, key: &ObjString) -> Option<&Entry> {
        if self.count == 0 {
            return None;
        }
        let index = Self::find_entry_index(&self.entries, self.capacity, key);
        let entry = &self.entries[index];
        if entry.key.is_none() {
            None
        } else {
            Some(entry)
        }
    }

    fn find_entry_mut(&mut self, key: &ObjString) -> Option<&mut Entry> {
        if self.count == 0 {
            return None;
        }
        let index = Self::find_entry_index(&self.entries, self.capacity, key);
        let entry = &mut self.entries[index];
        if entry.key.is_none() {
            None
        } else {
            Some(entry)
        }
    }

    fn table_set(&mut self, key: ObjString, value: Value) -> bool {
        if (self.count + 1) as f64 > (self.capacity * TABLE_MAX_LOAD) as f64 {
            let capacity = grow_capacity(self.capacity);
            self.adjust_capacity(capacity);
        }

        let entry = self.find_entry_mut(&key);
        if let Some(entry) = entry {
            entry.value = value;
            false
        } else {
            let index = Self::find_entry_index(&self.entries, self.capacity, &key);
            self.entries[index].key = Some(key);
            self.entries[index].value = value;
            self.count += 1;
            true
        }
    }

    fn table_get(&self, key: &ObjString) -> Option<Value> {
        if self.count == 0 {
            return None;
        }
        if let Some(entry) = self.find_entry(key) {
            Some(entry.value)
        } else {
            None
        }
    }

    fn table_delete(&mut self, key: &ObjString) -> bool {
        if self.count == 0 {
            return false;
        }
        if let Some(entry) = self.find_entry_mut(key) {
            entry.key = None;
            entry.value = Value::Bool(true);
            true
        } else {
            false
        }
    }

    fn table_add_all(&mut self, from: &Table) {
        for i in 0..from.capacity as usize {
            let entry = &from.entries[i];
            if entry.key.is_some() {
                self.table_set(entry.key.as_ref().unwrap().clone(), entry.value);
            }
        }
    }
}

#[derive(Clone)]
enum Value {
    Nil,
    Bool(bool),
    // Other value variants can be added here.
}

fn grow_capacity(capacity: i32) -> i32 {
    if capacity < 8 {
        8
    } else {
        capacity * 2
    }
}

fn main() {
    let mut strings = Table::new();
    // Initialize the 'strings' table.
    strings.init_table();

    // Example usage:
    let key = ObjString::new("example", 7, hash_string("example", 7));
    let value = Value::Bool(true);
    strings.table_set(key, value);
    if let Some(value) = strings.table_get(&ObjString::new("example", 7, hash_string("example", 7))) {
        match value {
            Value::Bool(b) => println!("Value: {}", b),
            _ => println!("Value is not a boolean."),
        }
    }
}

fn hash_string(key: &str, length: i32) -> u32 {
    let mut hash = 216_613_6261;
    for i in 0..length as usize {
        hash ^= key.as_bytes()[i] as u32;
        hash = hash.wrapping_mul(16_777_619);
    }
    hash
}
