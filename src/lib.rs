use kdl::{KdlNode, KdlValue};
use std::collections::HashMap;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KdlValueBuilder;
#[derive(Debug, Clone, PartialEq)]
pub struct KdlValuesBuilder {
    pub vals: Vec<KdlValue>,
    pub v: KdlValueBuilder,
}
#[derive(Debug, Clone, PartialEq)]
pub struct KdlPropertiesBuilder {
    pub props: HashMap<String, KdlValue>,
    pub v: KdlValueBuilder,
}
#[derive(Debug, Clone, PartialEq)]
pub struct KdlNodeBuilder {
    pub n: String,
    pub v: KdlValuesBuilder,
    pub p: KdlPropertiesBuilder,
    pub c: Vec<KdlNodeBuilder>,
}

impl KdlValueBuilder {
    pub fn str(self, value: String) -> KdlValue {
        KdlValue::String(value)
    }
    pub fn s(self, value: &str) -> KdlValue {
        self.str(value.to_string())
    }

    pub fn int(self, value: i64) -> KdlValue {
        KdlValue::Int(value)
    }
    pub fn i(self, value: i64) -> KdlValue {
        self.int(value)
    }

    pub fn flt(self, value: f64) -> KdlValue {
        KdlValue::Float(value)
    }
    pub fn f(self, value: f64) -> KdlValue {
        self.flt(value)
    }

    pub fn bool(self, value: bool) -> KdlValue {
        KdlValue::Boolean(value)
    }
    pub fn b(self, value: bool) -> KdlValue {
        self.bool(value)
    }
    pub fn y(self) -> KdlValue {
        self.bool(true)
    }
    pub fn n(self) -> KdlValue {
        self.bool(false)
    }

    pub fn nul(self) -> KdlValue {
        KdlValue::Null
    }
    pub fn nil(self) -> KdlValue {
        self.nul()
    }
}

impl KdlValuesBuilder {
    pub fn new() -> Self {
        Self {
            vals: Vec::new(),
            v: KdlValueBuilder,
        }
    }
    pub fn build(self) -> Vec<KdlValue> {
        self.vals
    }
    pub fn reset(&mut self) -> &mut Self {
        self.vals.clear();
        self
    }

    pub fn add(&mut self, val: KdlValue) {
        self.vals.push(val);
    }
    pub fn rem(&mut self, index: usize) {
        self.vals.remove(index);
    }
    pub fn set(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.vals = vals;
        self
    }

    pub fn join(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.vals.extend(vals);
        self
    }
    pub fn extend(&mut self, other: Self) -> &mut Self {
        self.vals.extend(other.vals);
        self
    }

    pub fn str(&mut self, val: &str) -> &mut Self {
        self.add(self.v.str(val.to_string()));
        self
    }
    pub fn s(&mut self, val: &str) -> &mut Self {
        self.add(self.v.s(val));
        self
    }

    pub fn int(&mut self, val: i64) -> &mut Self {
        self.add(self.v.int(val));
        self
    }
    pub fn i(&mut self, val: i64) -> &mut Self {
        self.add(self.v.i(val));
        self
    }

    pub fn flt(&mut self, val: f64) -> &mut Self {
        self.add(self.v.flt(val));
        self
    }
    pub fn f(&mut self, val: f64) -> &mut Self {
        self.add(self.v.f(val));
        self
    }

    pub fn nul(&mut self) -> &mut Self {
        self.add(self.v.nul());
        self
    }
    pub fn nil(&mut self) -> &mut Self {
        self.add(self.v.nil());
        self
    }

    pub fn bool(&mut self, val: bool) -> &mut Self {
        self.add(self.v.bool(val));
        self
    }
    pub fn b(&mut self, val: bool) -> &mut Self {
        self.add(self.v.b(val));
        self
    }
    pub fn y(&mut self) -> &mut Self {
        self.add(self.v.y());
        self
    }
    pub fn n(&mut self) -> &mut Self {
        self.add(self.v.n());
        self
    }
}

impl KdlPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            props: HashMap::new(),
            v: KdlValueBuilder,
        }
    }
    pub fn build(self) -> HashMap<String, KdlValue> {
        self.props
    }
    pub fn reset(&mut self) -> &mut Self {
        self.props.clear();
        self
    }

    pub fn add(&mut self, key: String, val: KdlValue) {
        self.props.insert(key, val);
    }
    pub fn rem(&mut self, key: &str) {
        self.props.remove(key);
    }
    pub fn set(&mut self, props: HashMap<String, KdlValue>) -> &mut Self {
        self.props = props;
        self
    }

    pub fn join(&mut self, props: HashMap<String, KdlValue>) -> &mut Self {
        self.props.extend(props);
        self
    }
    pub fn extend(&mut self, other: Self) -> &mut Self {
        self.props.extend(other.props);
        self
    }

    pub fn str(&mut self, id: &str, val: &str) -> &mut Self {
        self.add(id.to_string(), self.v.str(val.to_string()));
        self
    }
    pub fn s(&mut self, id: &str, val: &str) -> &mut Self {
        self.add(id.to_string(), self.v.s(val));
        self
    }

    pub fn int(&mut self, id: &str, val: i64) -> &mut Self {
        self.add(id.to_string(), self.v.int(val));
        self
    }
    pub fn i(&mut self, id: &str, val: i64) -> &mut Self {
        self.add(id.to_string(), self.v.i(val));
        self
    }

    pub fn flt(&mut self, id: &str, val: f64) -> &mut Self {
        self.add(id.to_string(), self.v.flt(val));
        self
    }
    pub fn f(&mut self, id: &str, val: f64) -> &mut Self {
        self.add(id.to_string(), self.v.f(val));
        self
    }

    pub fn bool(&mut self, id: &str, val: bool) -> &mut Self {
        self.add(id.to_string(), self.v.bool(val));
        self
    }
    pub fn b(&mut self, key: &str, value: bool) -> &mut Self {
        self.add(key.to_string(), self.v.b(value));
        self
    }
    pub fn y(&mut self, id: &str) -> &mut Self {
        self.add(id.to_string(), self.v.y());
        self
    }
    pub fn n(&mut self, id: &str) -> &mut Self {
        self.add(id.to_string(), self.v.n());
        self
    }

    pub fn nul(&mut self, id: &str) -> &mut Self {
        self.add(id.to_string(), self.v.nul());
        self
    }
    pub fn nil(&mut self, id: &str) -> &mut Self {
        self.add(id.to_string(), self.v.nil());
        self
    }
}

impl KdlNodeBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            n: name.to_string(),
            v: KdlValuesBuilder::new(),
            p: KdlPropertiesBuilder::new(),
            c: Vec::new(),
        }
    }
    pub fn build(self) -> KdlNode {
        KdlNode {
            name: self.n,
            values: self.v.build(),
            properties: self.p.build(),
            children: self.c.into_iter().map(|c| c.build()).collect(),
        }
    }
    pub fn reset(&mut self) -> &mut Self {
        self.reset_values();
        self.reset_properties();
        self.reset_children();
        self
    }
    pub fn extend(&mut self, val: KdlNodeBuilder) -> &mut Self {
        self.v.extend(val.v);
        self.p.extend(val.p);
        self.c.extend(val.c.into_iter());
        self
    }
    pub fn extend_children(&mut self, val: KdlNodeBuilder) -> &mut Self {
        self.c.extend(val.c.into_iter());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.n = name.to_string();
        self
    }
    pub fn reset_values(&mut self) -> &mut Self {
        self.v.reset();
        self
    }
    pub fn reset_properties(&mut self) -> &mut Self {
        self.p.reset();
        self
    }

    pub fn reset_child_values(&mut self) -> &mut Self {
        // run reset on each child node
        for c in &mut self.c {
            c.reset_values();
        }
        self
    }
    pub fn reset_child_properties(&mut self) -> &mut Self {
        // run reset on each child node
        for c in &mut self.c {
            c.reset_properties();
        }
        self
    }

    pub fn reset_child(&mut self, child: usize) -> &mut Self {
        self.c[child].reset();
        self
    }
    pub fn reset_children(&mut self) -> &mut Self {
        // run reset on each child node
        for c in &mut self.c {
            c.reset();
        }
        self
    }

    pub fn add(&mut self, val: KdlValue) -> &mut Self {
        self.v.add(val);
        self
    }
    pub fn val(&mut self, val: KdlValue) -> &mut Self {
        self.v.add(val);
        self
    }
    pub fn rem(&mut self, index: usize) -> &mut Self {
        self.v.rem(index);
        self
    }
    pub fn join(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.v.join(vals);
        self
    }
    pub fn vals(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        for val in vals {
            self.v.add(val.into());
        }
        self
    }
    pub fn value(&mut self, index: usize, val: KdlValue) -> &mut Self {
        self.v.rem(index);
        self.v.add(val);
        self
    }
    pub fn set(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.v.set(vals);
        self
    }
    pub fn values(&mut self, vals: Vec<KdlValue>) -> &mut Self {
        self.v.vals = vals;
        self
    }

    pub fn put(&mut self, key: &str, val: KdlValue) -> &mut Self {
        self.p.add(key.to_owned(), val);
        self
    }
    pub fn prop(&mut self, key: String, val: KdlValue) -> &mut Self {
        self.p.add(key, val);
        self
    }
    pub fn props(&mut self, props: HashMap<String, KdlValue>) -> &mut Self {
        for (key, val) in props {
            self.p.add(key, val);
        }
        self
    }
    pub fn property(&mut self, key: String, val: KdlValue) -> &mut Self {
        self.p.rem(&key);
        self.p.add(key, val);
        self
    }
    pub fn properties(&mut self, props: HashMap<String, KdlValue>) -> &mut Self {
        self.p.props = props;
        self
    }

    pub fn child(&mut self, child: KdlNodeBuilder) -> &mut Self {
        self.c.push(child);
        self
    }
    pub fn children(&mut self, children: Vec<KdlNodeBuilder>) -> &mut Self {
        self.c.extend(children);
        self
    }
    pub fn set_child(&mut self, index: usize, child: KdlNodeBuilder) -> &mut Self {
        self.c.remove(index);
        self.c.insert(index, child);
        self
    }
    pub fn set_children(&mut self, children: Vec<KdlNodeBuilder>) -> &mut Self {
        self.c = children;
        self
    }
    pub fn remove_child(&mut self, child: usize) -> &mut Self {
        self.c.remove(child);
        self
    }
    pub fn remove_children(&mut self) -> &mut Self {
        self.c.clear();
        self
    }
    pub fn str(&mut self, val: &str) -> &mut Self {
        self.v.str(val);
        self
    }
    pub fn s(&mut self, val: &str) -> &mut Self {
        self.v.s(val);
        self
    }

    pub fn int(&mut self, val: i64) -> &mut Self {
        self.v.int(val);
        self
    }
    pub fn i(&mut self, val: i64) -> &mut Self {
        self.v.i(val);
        self
    }

    pub fn flt(&mut self, val: f64) -> &mut Self {
        self.v.flt(val);
        self
    }
    pub fn f(&mut self, val: f64) -> &mut Self {
        self.v.f(val);
        self
    }

    pub fn bool(&mut self, val: bool) -> &mut Self {
        self.v.bool(val);
        self
    }
    pub fn b(&mut self, val: bool) -> &mut Self {
        self.v.b(val);
        self
    }
    pub fn y(&mut self) -> &mut Self {
        self.v.y();
        self
    }
    pub fn n(&mut self) -> &mut Self {
        self.v.n();
        self
    }

    pub fn null(&mut self) -> &mut Self {
        self.v.nul();
        self
    }
    pub fn nul(&mut self) -> &mut Self {
        self.v.nul();
        self
    }
    pub fn nil(&mut self) -> &mut Self {
        self.v.nil();
        self
    }
}
