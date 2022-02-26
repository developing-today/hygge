#![allow(dead_code)]

use kdl::{KdlNode, KdlValue};
use std::collections::HashMap;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
struct KdlValueBuilder;
impl KdlValueBuilder {
    fn str(self, value: String) -> KdlValue {
        KdlValue::String(value)
    }
    fn s(self, value: &str) -> KdlValue {
        self.str(value.to_string())
    }

    fn int(self, value: i64) -> KdlValue {
        KdlValue::Int(value)
    }
    fn i(self, value: i64) -> KdlValue {
        self.int(value)
    }

    fn flt(self, value: f64) -> KdlValue {
        KdlValue::Float(value)
    }
    fn f(self, value: f64) -> KdlValue {
        self.flt(value)
    }

    fn bool(self, value: bool) -> KdlValue {
        KdlValue::Boolean(value)
    }
    fn b(self, value: bool) -> KdlValue {
        self.bool(value)
    }
    fn y(self) -> KdlValue {
        self.bool(true)
    }
    fn n(self) -> KdlValue {
        self.bool(false)
    }

    fn nul(self) -> KdlValue {
        KdlValue::Null
    }
    fn nil(self) -> KdlValue {
        self.nul()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct KdlValuesBuilder {
    vals: Vec<KdlValue>,
    v: KdlValueBuilder,
}
impl KdlValuesBuilder {
    fn new() -> Self {
        Self {
            vals: Vec::new(),
            v: KdlValueBuilder,
        }
    }
    fn build(self) -> Vec<KdlValue> {
        self.vals
    }
    fn reset(&mut self) -> &mut Self {
        self.vals.clear();
        self
    }

    fn add(&mut self, val: KdlValue) {
        self.vals.push(val);
    }
    fn rem(&mut self, index: usize) {
        self.vals.remove(index);
    }
    fn set(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.vals = vals;
        self
    }

    fn join(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.vals.extend(vals);
        self
    }
    fn extend(&mut self, other: Self) -> &mut Self {
        self.vals.extend(other.vals);
        self
    }

    fn str(&mut self, val: &str) -> &mut Self {
        self.add(self.v.str(val.to_string()));
        self
    }
    fn s(&mut self, val: &str) -> &mut Self {
        self.add(self.v.s(val));
        self
    }

    fn int(&mut self, val: i64) -> &mut Self {
        self.add(self.v.int(val));
        self
    }
    fn i(&mut self, val: i64) -> &mut Self {
        self.add(self.v.i(val));
        self
    }

    fn flt(&mut self, val: f64) -> &mut Self {
        self.add(self.v.flt(val));
        self
    }
    fn f(&mut self, val: f64) -> &mut Self {
        self.add(self.v.f(val));
        self
    }

    fn nul(&mut self) -> &mut Self {
        self.add(self.v.nul());
        self
    }
    fn nil(&mut self) -> &mut Self {
        self.add(self.v.nil());
        self
    }

    fn bool(&mut self, val: bool) -> &mut Self {
        self.add(self.v.bool(val));
        self
    }
    fn b(&mut self, val: bool) -> &mut Self {
        self.add(self.v.b(val));
        self
    }
    fn y(&mut self) -> &mut Self {
        self.add(self.v.y());
        self
    }
    fn n(&mut self) -> &mut Self {
        self.add(self.v.n());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
struct KdlPropertiesBuilder {
    props: HashMap<String, KdlValue>,
    v: KdlValueBuilder,
}
impl KdlPropertiesBuilder {
    fn new() -> Self {
        Self {
            props: HashMap::new(),
            v: KdlValueBuilder,
        }
    }
    fn build(self) -> HashMap<String, KdlValue> {
        self.props
    }
    fn reset(&mut self) -> &mut Self {
        self.props.clear();
        self
    }

    fn add(&mut self, key: String, val: KdlValue) {
        self.props.insert(key, val);
    }
    fn rem(&mut self, key: &str) {
        self.props.remove(key);
    }
    fn set(&mut self, props: HashMap<String, KdlValue>) -> &mut Self {
        self.props = props;
        self
    }

    fn join(&mut self, props: HashMap<String, KdlValue>) -> &mut Self {
        self.props.extend(props);
        self
    }
    fn extend(&mut self, other: Self) -> &mut Self {
        self.props.extend(other.props);
        self
    }

    fn str(&mut self, id: &str, val: &str) -> &mut Self {
        self.add(id.to_string(), self.v.str(val.to_string()));
        self
    }
    fn s(&mut self, id: &str, val: &str) -> &mut Self {
        self.add(id.to_string(), self.v.s(val));
        self
    }

    fn int(&mut self, id: &str, val: i64) -> &mut Self {
        self.add(id.to_string(), self.v.int(val));
        self
    }
    fn i(&mut self, id: &str, val: i64) -> &mut Self {
        self.add(id.to_string(), self.v.i(val));
        self
    }

    fn flt(&mut self, id: &str, val: f64) -> &mut Self {
        self.add(id.to_string(), self.v.flt(val));
        self
    }
    fn f(&mut self, id: &str, val: f64) -> &mut Self {
        self.add(id.to_string(), self.v.f(val));
        self
    }

    fn bool(&mut self, id: &str, val: bool) -> &mut Self {
        self.add(id.to_string(), self.v.bool(val));
        self
    }
    fn b(&mut self, key: &str, value: bool) -> &mut Self {
        self.add(key.to_string(), self.v.b(value));
        self
    }
    fn y(&mut self, id: &str) -> &mut Self {
        self.add(id.to_string(), self.v.y());
        self
    }
    fn n(&mut self, id: &str) -> &mut Self {
        self.add(id.to_string(), self.v.n());
        self
    }

    fn nul(&mut self, id: &str) -> &mut Self {
        self.add(id.to_string(), self.v.nul());
        self
    }
    fn nil(&mut self, id: &str) -> &mut Self {
        self.add(id.to_string(), self.v.nil());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
struct KdlNodeBuilder {
    n: String,
    v: KdlValuesBuilder,
    p: KdlPropertiesBuilder,
    c: Vec<KdlNodeBuilder>,
}
impl KdlNodeBuilder {
    fn new(name: &str) -> Self {
        Self {
            n: name.to_string(),
            v: KdlValuesBuilder::new(),
            p: KdlPropertiesBuilder::new(),
            c: Vec::new(),
        }
    }
    fn build(self) -> KdlNode {
        KdlNode {
            name: self.n,
            values: self.v.build(),
            properties: self.p.build(),
            children: self.c.into_iter().map(|c| c.build()).collect(),
        }
    }
    fn reset(&mut self) -> &mut Self {
        self.reset_values();
        self.reset_properties();
        self.reset_children();
        self
    }
    fn extend(&mut self, val: KdlNodeBuilder) -> &mut Self {
        self.v.extend(val.v);
        self.p.extend(val.p);
        self.c.extend(val.c.into_iter());
        self
    }
    fn extend_children(&mut self, val: KdlNodeBuilder) -> &mut Self {
        self.c.extend(val.c.into_iter());
        self
    }

    fn name(&mut self, name: &str) -> &mut Self {
        self.n = name.to_string();
        self
    }
    fn reset_values(&mut self) -> &mut Self {
        self.v.reset();
        self
    }
    fn reset_properties(&mut self) -> &mut Self {
        self.p.reset();
        self
    }

    fn reset_child_values(&mut self) -> &mut Self {
        // run reset on each child node
        for c in &mut self.c {
            c.reset_values();
        }
        self
    }
    fn reset_child_properties(&mut self) -> &mut Self {
        // run reset on each child node
        for c in &mut self.c {
            c.reset_properties();
        }
        self
    }

    fn reset_child(&mut self, child: usize) -> &mut Self {
        self.c[child].reset();
        self
    }
    fn reset_children(&mut self) -> &mut Self {
        // run reset on each child node
        for c in &mut self.c {
            c.reset();
        }
        self
    }

    fn add(&mut self, val: KdlValue) -> &mut Self {
        self.v.add(val);
        self
    }
    fn val(&mut self, val: KdlValue) -> &mut Self {
        self.v.add(val);
        self
    }
    fn rem(&mut self, index: usize) -> &mut Self {
        self.v.rem(index);
        self
    }
    fn join(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.v.join(vals);
        self
    }
    fn vals(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        for val in vals {
            self.v.add(val.into());
        }
        self
    }
    fn value(&mut self, index: usize, val: KdlValue) -> &mut Self {
        self.v.rem(index);
        self.v.add(val);
        self
    }
    fn set(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.v.set(vals);
        self
    }
    fn values(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.v.vals = vals;
        self
    }

    fn put(&mut self, key: &str, val: KdlValue) -> &mut Self {
        self.p.add(key.to_owned(), val);
        self
    }
    fn prop(&mut self, key: String, val: KdlValue) -> &mut Self {
        self.p.add(key, val);
        self
    }
    fn props(&mut self, props: HashMap<String, KdlValue>) -> &mut Self {
        for (key, val) in props {
            self.p.add(key, val);
        }
        self
    }
    fn property(&mut self, key: String, val: KdlValue) -> &mut Self {
        self.p.rem(&key);
        self.p.add(key, val);
        self
    }
    fn properties(&mut self, props: HashMap<String, KdlValue>) -> &mut Self {
        self.p.props = props;
        self
    }

    fn child(&mut self, child: KdlNodeBuilder) -> &mut Self {
        self.c.push(child);
        self
    }
    fn children(&mut self, children: Vec<KdlNodeBuilder>) -> &mut Self {
        self.c.extend(children);
        self
    }
    fn set_child(&mut self, index: usize, child: KdlNodeBuilder) -> &mut Self {
        self.c.remove(index);
        self.c.insert(index, child);
        self
    }
    fn set_children(&mut self, children: Vec<KdlNodeBuilder>) -> &mut Self {
        self.c = children;
        self
    }
    fn remove_child(&mut self, child: usize) -> &mut Self {
        self.c.remove(child);
        self
    }
    fn remove_children(&mut self) -> &mut Self {
        self.c.clear();
        self
    }
    fn str(&mut self, val: &str) -> &mut Self {
        self.v.str(val);
        self
    }
    fn s(&mut self, val: &str) -> &mut Self {
        self.v.s(val);
        self
    }

    fn int(&mut self, val: i64) -> &mut Self {
        self.v.int(val);
        self
    }
    fn i(&mut self, val: i64) -> &mut Self {
        self.v.i(val);
        self
    }

    fn flt(&mut self, val: f64) -> &mut Self {
        self.v.flt(val);
        self
    }
    fn f(&mut self, val: f64) -> &mut Self {
        self.v.f(val);
        self
    }

    fn bool(&mut self, val: bool) -> &mut Self {
        self.v.bool(val);
        self
    }
    fn b(&mut self, val: bool) -> &mut Self {
        self.v.b(val);
        self
    }
    fn y(&mut self) -> &mut Self {
        self.v.y();
        self
    }
    fn n(&mut self) -> &mut Self {
        self.v.n();
        self
    }

    fn null(&mut self) -> &mut Self {
        self.v.nul();
        self
    }
    fn nul(&mut self) -> &mut Self {
        self.v.nul();
        self
    }
    fn nil(&mut self) -> &mut Self {
        self.v.nil();
        self
    }
}

fn main() {
    let v = KdlValueBuilder;
    let mut kdl = KdlNodeBuilder::new("hygge");
    let mut childs = KdlNodeBuilder::new("child");
    childs
        .bool(true)
        .prop("uuid".to_string(), v.str("123".to_string()).to_owned());
    kdl.nil()
        .nil()
        .y()
        .n()
        .child(childs)
        .str("hello")
        .child(KdlNodeBuilder::new("child2"));
    println!("{}", kdl.build());
}
