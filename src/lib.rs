use core::panic;
pub use kdl::{KdlNode, KdlValue};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HashSetMap<T> {
    register: HashMap<Rc<u64>, Rc<T>>,
}

pub struct HashSetMapBuilder<T> {
    hash_register: HashSetMap<T>,
    hasher: Option<Rc<dyn Hasher>>,
}
impl<'a, T> HashSetMapBuilder<T>
where
    T: Hash + Eq + Clone + Default + ToString,
    HashSetMapBuilder<T>: Default,
{
    pub fn insert(&mut self, hasher: &'a mut dyn Hasher, input: T) -> Option<Rc<T>> {
        if self.hasher.is_none() {
            panic!("HashSetMapBuilder::insert() -> hasher is None")
        };
        hasher.write(input.to_string().as_bytes());
        let key = hasher.finish();
        self.hash_register
            .register
            .insert(Rc::new(key), Rc::new(input))
    }
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_with_capacity(capacity: usize) -> Self {
        let mut hash_register = HashSetMap::default();
        hash_register.register = HashMap::with_capacity(capacity);
        Self {
            hash_register,
            hasher: None,
        }
    }
    pub fn new_with_hasher(hasher: Rc<dyn Hasher>) -> Self {
        Self {
            hash_register: Default::default(),
            hasher: Some(hasher),
        }
    }

    pub fn new_with_capacity_and_hasher(capacity: usize, hasher: Rc<dyn Hasher>) -> Self {
        let mut hash_register = HashSetMap::default();
        hash_register.register = HashMap::with_capacity_and_hasher(capacity, Default::default());
        Self {
            hash_register,
            hasher: Some(hasher),
        }
    }
    pub fn build(self) -> HashMap<Rc<u64>, Rc<T>> {
        self.hash_register.register
    }

    pub fn with_hasher(mut self, hasher: Rc<dyn Hasher>) -> Self {
        self.hasher = Some(hasher);
        self
    }
    pub fn hasher(&mut self, hasher: Rc<dyn Hasher>) -> &mut Self {
        self.hasher = Some(hasher);
        self
    }

    pub fn len(&self) -> usize {
        self.hash_register.register.len()
    }
    pub fn is_empty(&self) -> bool {
        self.hash_register.register.is_empty()
    }
    pub fn clear(&mut self) {
        self.hash_register.register.clear();
    }
}
impl<T> Default for HashSetMapBuilder<T>
where
    T: Hash + Eq + Clone + Default + ToString,
{
    fn default() -> Self {
        Self {
            hash_register: HashSetMap {
                register: HashMap::new(),
            },
            hasher: Some(Rc::new(DefaultHasher::new())),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KdlValueBuilder;

// impl Clone for KdlValueBuilder {
//     fn clone(&self) -> Self {
//         KdlValueBuilder {}
//     }
// }

#[derive(Debug, PartialEq)]
pub struct KdlValuesProxy(pub Vec<KdlValue>);
#[derive(Debug, PartialEq)]

pub struct KdlValuesBuilder {
    pub vals: KdlValuesProxy,
    pub v: KdlValueBuilder,
}

#[derive(Debug, PartialEq)]
pub struct KdlPropertiesProxy(pub HashMap<String, KdlValue>);

#[derive(Debug, PartialEq)]
pub struct KdlPropertiesBuilder {
    pub props: KdlPropertiesProxy,
    pub v: KdlValueBuilder,
}

#[derive(Debug, PartialEq)]
pub struct KdlNodeBuilder {
    pub n: String,
    pub v: KdlValuesBuilder,
    pub p: KdlPropertiesBuilder,
    pub c: KdlChildrenProxy,
}

#[derive(Debug, PartialEq)]
pub struct KdlChildrenProxy(Vec<KdlNodeBuilder>);
impl KdlChildrenProxy {
    pub fn new() -> Self {
        KdlChildrenProxy(Vec::new())
    }
    // pub fn build(&self) -> Vec<KdlNode> {

    //         .collect()
    // }
    pub fn build(mut self) -> Vec<KdlNode> {
        self.0.drain(..).map(|n| n.build()).collect()
    }

    // pub fn add(mut self, n: KdlNodeBuilder) -> Self {
    //     self.0.push(n);
    //     self
    // }
}

trait KdlValueBuilderBuilder {
    fn builder() -> KdlValueBuilder {
        KdlValueBuilder::new()
    }
    fn builder_vec() -> KdlValuesBuilder {
        KdlValuesBuilder::new()
    }
}
impl KdlValueBuilderBuilder for KdlValue {}

trait KdlPropertiesBuilderBuilder {
    fn builder() -> KdlPropertiesBuilder {
        return KdlPropertiesBuilder::new();
    }
}
impl KdlPropertiesBuilderBuilder for &mut HashMap<&&mut str, KdlValue> {}

trait KdlNodeBuilderBuilder {
    fn builder(name: &str) -> KdlNodeBuilder {
        KdlNodeBuilder::new(&mut name.to_string())
    }
}
impl KdlNodeBuilderBuilder for KdlNode {}

impl KdlValueBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn clone(mut self) -> Self {
        self
    }

    pub fn str(mut self, val: &str) -> KdlValue {
        KdlValue::String(val.to_string())
    }
    pub fn s(mut self, val: &mut str) -> KdlValue {
        self.str(val)
    }

    pub fn int(mut self, val: i64) -> KdlValue {
        KdlValue::Int(val)
    }
    pub fn i(mut self, val: i64) -> KdlValue {
        self.int(val)
    }

    pub fn flt(mut self, val: f64) -> KdlValue {
        KdlValue::Float(val)
    }
    pub fn f(mut self, val: f64) -> KdlValue {
        self.flt(val)
    }

    pub fn bool(mut self, val: bool) -> KdlValue {
        KdlValue::Boolean(val)
    }
    pub fn b(mut self, val: bool) -> KdlValue {
        self.bool(val)
    }
    pub fn y(mut self) -> KdlValue {
        self.bool(true)
    }
    pub fn n(mut self) -> KdlValue {
        self.bool(false)
    }

    pub fn nul(mut self) -> KdlValue {
        KdlValue::Null
    }
    pub fn nil(mut self) -> KdlValue {
        self.nul()
    }
}
// // implement clone which clones all children correctly
// impl Clone for KdlValuesBuilder {
//     fn clone(&self) -> Self {
//         Self {
//             vals: &mut self.vals.0.clone(),
//             v: &mut self.v.clone(),
//         }
//     }

//     fn clone_from(&mut self, source: &Self) {
//         self.vals.0.clone_from(&source.vals.0);
//         self.v.clone_from(&source.v);
//     }
// }
// impl Clone for KdlValueBuilder {
//     fn clone(&self) -> Self {
//         Self
//     }
// }
// impl Clone for KdlValues {
//     fn clone(&self) -> Self {
//         let clone = self.clone().0;
//         KdlValues(clone)
//     }
// }

// impl Clone for KdlValuesBuilder {
//     fn clone(&self) -> Self {
//         let mut vals = Vec::new();
//         for val in self {
//             vals.push(val);
//         }
//         vals
//     }
// }

impl KdlValuesProxy {
    pub fn add(mut self, val: KdlValue) -> Self {
        self.0.push(val);
        self
    }
    pub fn rem(mut self, index: usize) -> Self {
        self.0.remove(index);
        self
    }
    pub fn clear(mut self) -> Self {
        self.0.clear();
        self
    }
    pub fn set(mut self, vals: &mut KdlValuesProxy) -> Self {
        self.0 = vals.0.clone();
        self
    }
    pub fn append(mut self, vals: &mut KdlValuesProxy) -> Self {
        self.0.append(&mut vals.0);
        self
    }
}
impl KdlValuesBuilder {
    pub fn new() -> Self {
        Self {
            vals: KdlValuesProxy(Vec::new()),
            v: KdlValueBuilder,
        }
    }
    pub fn build(&self) -> Vec<KdlValue> {
        let value = self.vals.0.clone();
        value.iter().map(|v| v.to_owned()).collect()
    }
    pub fn reset(mut self) -> Self {
        self.vals.0.clear();
        self
    }

    pub fn add(mut self, val: KdlValue) -> Self {
        self.vals.0.push(val);
        self
    }
    pub fn rem(mut self, index: usize) -> Self {
        self.vals.0.remove(index);
        self
    }
    pub fn clear(mut self) -> Self {
        self.vals.0.clear();
        self
    }
    pub fn set(mut self, vals: &mut KdlValuesProxy) -> Self {
        self.vals.0 = vals.0.clone();
        self
    }
    pub fn join(mut self, vals: &mut KdlValuesProxy) -> Self {
        self.vals.0.append(&mut vals.0);
        self
    }
    pub fn extend(mut self, other: Self) -> Self {
        self.vals.0.extend(other.vals.0);
        self
    }

    pub fn str(mut self, val: &mut str) -> Self {
        let v = &self.v;
        let value = v.str(val);
        self.add(value)
    }
    pub fn s(mut self, val: &mut str) -> Self {
        let value = self.v.s(val);
        self.add(value)
    }

    pub fn int(mut self, val: i64) -> Self {
        let value = self.v.int(val);
        self.add(value)
    }
    pub fn i(mut self, val: i64) -> Self {
        let value = self.v.i(val);
        self.add(value)
    }

    pub fn flt(mut self, val: f64) -> Self {
        let value = self.v.flt(val);
        self.add(value)
    }
    pub fn f(mut self, val: f64) -> Self {
        let value = self.v.f(val);
        self.add(value)
    }

    pub fn nul(mut self) -> Self {
        let val = self.v.nul();
        self.add(val)
    }
    pub fn nil(mut self) -> Self {
        let value = self.v.nil();
        self.add(value)
    }

    pub fn bool(mut self, val: bool) -> Self {
        let value = self.v.bool(val);
        self.add(value)
    }
    pub fn b(mut self, val: bool) -> Self {
        let value = self.v.b(val);
        self.add(value)
    }
    pub fn y(mut self) -> Self {
        let val = self.v.y();
        self.add(val)
    }
    pub fn n(mut self) -> Self {
        let val = self.v.n();
        self.add(val)
    }
}

//   https://crates.io/crates/derivative
//   #[derive(Default)]
//   struct ParametersWithDefault {
//     ...
//   }

impl KdlPropertiesProxy {
    fn new() -> Self {
        KdlPropertiesProxy(HashMap::new())
    }
}
impl KdlPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            props: KdlPropertiesProxy::new(),
            v: KdlValueBuilder,
        }
    }
    pub fn build(mut self) -> HashMap<String, KdlValue> {
        // properties map must be dereferenced and cloned into owned types
        let mut output = HashMap::new();
        for (key, value) in self.props.0.iter() {
            output.insert(key.to_string(), value.clone());
        }
        output
    }

    pub fn reset(mut self) -> Self {
        self.props.0.clear();
        self
    }

    pub fn add(mut self, key: &mut str, val: KdlValue) -> Self {
        self.props.0.insert(key.to_string(), val);
        self
    }
    pub fn rem(mut self, key: &mut str) -> Self {
        self.props.0.remove(key);
        self
    }
    pub fn set(mut self, props: &mut HashMap<&mut str, KdlValue>) -> Self {
        //     self.props.0.clear();
        //     for (key, &value) in props.iter() {
        //         self.props.0.insert(key.to_string(), value);
        //     }
        //     self
        let mut new = HashMap::new();
        for (key, value) in props.iter() {
            new.insert(key.to_string(), value.clone());
        }
        self.props.0.clear();
        self.props.0.extend(new);
        self
    }

    pub fn join(mut self, props: &mut KdlPropertiesProxy) -> Self {
        //     self.props.0.clear();
        //     for (key, &value) in props.iter() {
        //         self.props.0.insert(key.to_string(), value);
        //     }
        //     self
        let mut new = HashMap::new();
        for (key, value) in props.0.iter() {
            new.insert(key.to_string(), value.clone());
        }
        self.props.0.extend(new);
        self
    }
    pub fn extend(mut self, other: KdlPropertiesBuilder) -> Self {
        let mut new = HashMap::new();
        for (key, value) in other.props.0.iter() {
            new.insert(key.to_string(), value.clone());
        }
        self.props.0.extend(new);
        self
    }

    pub fn str(mut self, id: &mut str, val: &mut str) -> Self {
        let string = self.v.str(val);
        self.add(id, string)
    }
    pub fn s(mut self, id: &mut str, val: &mut str) -> Self {
        let value = self.v.s(val);
        self.add(id, value)
    }

    pub fn int(mut self, id: &mut str, val: i64) -> Self {
        let value = self.v.int(val);
        self.add(id, value)
    }
    pub fn i(mut self, id: &mut str, val: i64) -> Self {
        let value = self.v.i(val);
        self.add(id, value)
    }

    pub fn flt(mut self, id: &mut str, val: f64) -> Self {
        let value = self.v.flt(val);
        self.add(id, value)
    }
    pub fn f(mut self, id: &mut str, val: f64) -> Self {
        let value = self.v.f(val);
        self.add(id, value)
    }

    pub fn bool(mut self, id: &mut str, val: bool) -> Self {
        let value = self.v.bool(val);
        self.add(id, value)
    }
    pub fn b(mut self, key: &mut str, val: bool) -> Self {
        let value = self.v.b(val);
        self.add(key, value)
    }
    pub fn y(mut self, id: &mut str) -> Self {
        let val = self.v.y();
        self.add(id, val)
    }
    pub fn n(mut self, id: &mut str) -> Self {
        let val = self.v.n();
        self.add(id, val)
    }

    pub fn nul(mut self, id: &mut str) -> Self {
        let val = self.v.nul();
        self.add(id, val)
    }
    pub fn nil(mut self, id: &mut str) -> Self {
        let val = self.v.nil();
        self.add(id, val)
    }
}
// impl Clone for KdlPropertiesBuilder {
//     fn clone(&self) -> Self {
//         Self {
//             props: &mut self.props.clone(),
//             v: &mut self.v.clone(),
//         }
//     }
// }
// impl Default for KdlValueBuilder {
//     fn default() -> Self {
//         Self {
//             v: &mut KdlValue::default(),
//         }
//     }
// }
// impl Clone for KdlNodeBuilder {
//     fn clone(&self) -> Self {
//         Self {
//             n: self.n,
//             v: &mut self.v.clone(),
//             p: &mut self.p.clone(),
//             c: self.c.clone(),
//         }
//     }
// }
impl Default for KdlPropertiesBuilder {
    fn default() -> Self {
        Self {
            props: KdlPropertiesProxy::new(),
            v: KdlValueBuilder,
        }
    }
}

impl KdlNodeBuilder {
    pub fn new(name: &mut str) -> Self {
        Self {
            n: name.to_string(),
            v: KdlValuesBuilder::new(),
            p: KdlPropertiesBuilder::new(),
            c: KdlChildrenProxy::new(),
        }
    }
    pub fn build(mut self) -> KdlNode {
        KdlNode {
            name: self.n.to_string(),
            values: self.v.build(),
            properties: self.p.build(),
            children: self.c.build(),
        }
    }
    pub fn reset(mut self) -> Self {
        self.reset_children().reset_values().reset_properties()
    }
    pub fn extend(mut self, val: KdlNodeBuilder) -> Self {
        self.c.0.extend(val.c.0);
        self.v = self.v.extend(val.v);
        self.p.props.0.extend(val.p.props.0);
        self
    }
    pub fn extend_children(mut self, val: &mut KdlNodeBuilder) -> Self {
        let output = Vec::new();
        self.c.0.extend(output);
        self
    }

    pub fn name(mut self, name: &mut str) -> Self {
        self.n = name.to_string();
        self
    }
    pub fn reset_values(mut self) -> Self {
        self.v = self.v.reset();
        self
    }
    pub fn reset_properties(mut self) -> Self {
        self.p = self.p.reset();
        self
    }

    pub fn reset_child(mut self, child: usize) -> Self {
        // self.c.0[child] = self.c.0[child].reset();
        self
    }
    pub fn reset_child_values(mut self, child: usize) -> Self {
        // self.c.0[child] = self.c.0[child].reset_values();
        self
    }
    pub fn reset_child_properties(mut self, child: usize) -> Self {
        // self.c.0[child] = self.c.0[child].reset_properties();
        self
    }
    pub fn reset_children(mut self) -> Self {
        // run reset on each child node
        // self.c.0 = self
        //     .c
        //     .0
        //     .iter()
        //     .map(|c| -> KdlNodeBuilder { c.reset() })
        //     .collect();
        self
    }
    pub fn reset_children_values(mut self) -> Self {
        // run reset on each child node
        self.c.0 = self.c.0.into_iter().map(|c| c.reset()).collect();
        self
    }
    pub fn reset_children_properties(mut self) -> Self {
        // run reset on each child node
        self.c.0 = self.c.0.into_iter().map(|c| c.reset()).collect();
        self
    }
    pub fn add(mut self, val: KdlValue) -> Self {
        self.v = self.v.add(val);
        self
    }
    pub fn val(mut self, val: KdlValue) -> Self {
        self.v = self.v.add(val);
        self
    }
    pub fn rem(mut self, index: usize) -> Self {
        self.v = self.v.rem(index);
        self
    }
    pub fn join(mut self, vals: &mut KdlValuesProxy) -> Self {
        self.v = self.v.join(vals);
        self
    }
    pub fn vals(mut self, vals: &mut KdlValuesProxy) -> Self {
        self.v = self.v.join(vals);
        self
    }
    pub fn value(mut self, index: usize, val: KdlValue) -> Self {
        self.v = self.v.rem(index);
        self.v = self.v.add(val);
        self
    }
    pub fn set(mut self, vals: &mut KdlValuesProxy) -> Self {
        self.v = self.v.set(vals);
        self
    }
    pub fn values(mut self, vals: &mut KdlValuesProxy) -> Self {
        self.v = self.v.set(vals);
        self
    }

    pub fn put(mut self, key: &mut str, val: KdlValue) -> Self {
        self.p = self.p.add(key, val);
        self
    }
    pub fn prop(mut self, key: &mut str, val: KdlValue) -> Self {
        self.p = self.p.add(key, val);
        self
    }
    pub fn props(mut self, props: &mut HashMap<&mut str, KdlValue>) -> Self {
        self.properties(props)
    }
    pub fn property(mut self, key: &mut str, val: KdlValue) -> Self {
        self.p = self.p.rem(key);
        self.p = self.p.add(key, val);
        self
    }

    /*
    turn into deque
    for each 2
                */
    pub fn properties(mut self, props: &mut HashMap<&mut str, KdlValue>) -> Self {
        // ensure types are all mutable use a decque tuple
        let mut keys = Vec::new();
        let mut vals: Vec<KdlValue> = Vec::new();
        for (key, val) in props {
            keys.push(key.to_string());
            // vals.push(val.clone());
            // error mismatched types
            // try something else
            vals.push(val.clone().into());
        }
        // make a vector decque of 2 tuple pairs
        let mut pairs = Vec::new();
        for i in 0..keys.len() {
            pairs.push((keys[i].clone(), vals[i].clone()));
        }
        // for each pair
        for (mut key, val) in pairs {
            // remove the key
            self.p = self.p.rem(&mut key);
            // add the new value
            self.p = self.p.add(&mut key, val);
        }
        self
    }

    pub fn child(mut self, child: KdlNodeBuilder) -> Self {
        self.c.0.push(child);
        self
    }
    pub fn children(mut self, children: Vec<KdlNodeBuilder>) -> Self {
        self.c.0.extend(children);
        self
    }
    pub fn insert_child(mut self, index: usize, child: KdlNodeBuilder) -> Self {
        self.c.0.insert(index, child);
        self
    }
    pub fn set_children(mut self, children: Vec<KdlNodeBuilder>) -> Self {
        self.c.0 = children;
        self
    }
    pub fn remove_child(mut self, child: usize) -> Self {
        self.c.0.remove(child);
        self
    }
    pub fn remove_children(mut self) -> Self {
        self.c.0.clear();
        self
    }
    pub fn str(mut self, val: &mut str) -> Self {
        self.v.vals.0.push(KdlValue::String(val.to_string()));
        self
    }
    pub fn s(mut self, val: &mut str) -> Self {
        self.v = self.v.s(val);
        self
    }

    pub fn int(mut self, val: i64) -> Self {
        self.v = self.v.int(val);
        self
    }
    pub fn i(mut self, val: i64) -> Self {
        self.v = self.v.i(val);
        self
    }

    pub fn flt(mut self, val: f64) -> Self {
        self.v = self.v.flt(val);
        self
    }
    pub fn f(mut self, val: f64) -> Self {
        self.v = self.v.f(val);
        self
    }

    pub fn bool(mut self, val: bool) -> Self {
        self.v = self.v.bool(val);
        self
    }
    pub fn b(mut self, val: bool) -> Self {
        self.v = self.v.b(val);
        self
    }
    pub fn y(mut self) -> Self {
        self.v = self.v.y();
        self
    }
    pub fn n(mut self) -> Self {
        self.v = self.v.n();
        self
    }

    pub fn null(mut self) -> Self {
        self.v = self.v.nul();
        self
    }
    pub fn nul(mut self) -> Self {
        self.v = self.v.nul();
        self
    }
    pub fn nil(mut self) -> Self {
        self.v = self.v.nil();
        self
    }
}
