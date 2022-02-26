use kdl::{KdlNode, KdlValue};
use std::collections::HashMap;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KdlValueBuilder;

#[derive(Debug, PartialEq)]
pub struct KdlValuesBuilder<'a> {
    pub vals: &'a mut Vec<&'a mut KdlValue>,
    pub v: &'a mut KdlValueBuilder,
}

#[derive(Debug, PartialEq)]
pub struct KdlPropertiesBuilder<'a> {
    pub props: &'a mut HashMap<&'a str, &'a KdlValue>,
    pub v: &'a mut KdlValueBuilder,
}

#[derive(Debug, PartialEq)]
pub struct KdlNodeBuilder<'a> {
    pub n: &'a str,
    pub v: &'a mut KdlValuesBuilder<'a>,
    pub p: &'a mut KdlPropertiesBuilder<'a>,
    pub c: &'a mut Vec<&'a mut KdlNodeBuilder<'a>>,
}

trait KdlValueBuilderBuilder<'a> {
    fn builder() -> KdlValueBuilder {
        KdlValueBuilder::new()
    }
    fn builder_vec() -> KdlValuesBuilder<'a> {
        KdlValuesBuilder::new()
    }
}
impl<'a> KdlValueBuilderBuilder<'a> for KdlValue {}

trait KdlPropertiesBuilderBuilder<'a> {
    fn builder() -> KdlPropertiesBuilder<'a> {
        return KdlPropertiesBuilder::new();
    }
}
impl<'a> KdlPropertiesBuilderBuilder<'a> for HashMap<&&'a str, KdlValue> {}

trait KdlNodeBuilderBuilder<'a> {
    fn builder(name: &'a str) -> KdlNodeBuilder {
        KdlNodeBuilder::new(name)
    }
}
impl<'a> KdlNodeBuilderBuilder<'a> for KdlNode {}

impl<'a> KdlValueBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn str(mut self, val: &'a str) -> &'a KdlValue {
        &KdlValue::String(val.to_string())
    }
    pub fn s(mut self, val: &'a str) -> &'a KdlValue {
        self.str(val)
    }

    pub fn int(mut self, val: i64) -> &'a KdlValue {
        &KdlValue::Int(val)
    }
    pub fn i(mut self, val: i64) -> &'a KdlValue {
        self.int(val)
    }

    pub fn flt(mut self, val: f64) -> &'a KdlValue {
        &KdlValue::Float(val)
    }
    pub fn f(mut self, val: f64) -> &'a KdlValue {
        self.flt(val)
    }

    pub fn bool(mut self, val: bool) -> &'a KdlValue {
        &KdlValue::Boolean(val)
    }
    pub fn b(mut self, val: bool) -> &'a KdlValue {
        self.bool(val)
    }
    pub fn y(mut self) -> &'a KdlValue {
        self.bool(true)
    }
    pub fn n(mut self) -> &'a KdlValue {
        self.bool(false)
    }

    pub fn nul(mut self) -> &'a KdlValue {
        &KdlValue::Null
    }
    pub fn nil(mut self) -> &'a KdlValue {
        self.nul()
    }
}

impl<'a> KdlValuesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            vals: &mut Vec::new(),
            v: &mut KdlValueBuilder,
        }
    }
    pub fn build(mut self) -> Vec<KdlValue> {
        // return a vector without references
        let output = Vec::new();
        for val in self.vals {
            output.push(val.clone().to_owned());
        }
        output
    }
    pub fn reset(mut self) -> Self {
        self.vals.clear();
        self
    }

    pub fn add(mut self, val: &'a KdlValue) -> Self {
        self.vals.push(val);
        self
    }
    pub fn rem(mut self, index: usize) -> Self {
        self.vals.remove(index);
        self
    }
    pub fn set(mut self, vals: &'a mut Vec<&mut 'a KdlValue>) -> Self {
        self.vals = vals;
        self
    }

    pub fn join(mut self, vals: &'a mut Vec<&'a mut KdlValue>) -> Self {
        self.vals.extend(vals);
        self
    }
    pub fn extend(mut self, other: &mut Self) -> Self {
        self.vals.extend(other.vals.clone());
        self
    }

    pub fn str(mut self, val: &'a str) -> Self {
        let value = self.v.str(val);
        self.add(value)
    }
    pub fn s(mut self, val: &'a str) -> Self {
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
impl<'a> KdlPropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            props: &mut HashMap::new(),
            v: &mut KdlValueBuilder,
        }
    }
    pub fn build(mut self) -> HashMap<String, KdlValue> {
        // properties map must be dereferenced and cloned into owned types
        let output = HashMap::new();
        for (key, value) in self.props.iter() {
            output.insert(key.to_string(), value.clone().to_owned());
        }
        output
    }

    pub fn reset(mut self) -> Self {
        self.props.clear();
        self
    }

    pub fn add(mut self, key: &'a str, val: &'a KdlValue) -> Self {
        self.props.insert(key, val);
        self
    }
    pub fn rem(mut self, key: &'a str) -> Self {
        self.props.remove(key);
        self
    }
    pub fn set(mut self, props: &'a mut HashMap<&'a str, &'a KdlValue>) -> Self {
        self.props = props;
        self
    }

    pub fn join(mut self, props: &'a HashMap<&'a str, &'a KdlValue>) -> Self {
        self.props.extend(props);
        self
    }
    pub fn extend(mut self, other: &mut Self) -> Self {
        self.props.extend(other.props.clone());
        self
    }

    pub fn str(mut self, id: &'a str, val: &'a str) -> Self {
        let string = self.v.str(val);
        self.add(id, string)
    }
    pub fn s(mut self, id: &'a str, val: &'a str) -> Self {
        let value = self.v.s(val);
        self.add(id, value)
    }

    pub fn int(mut self, id: &'a str, val: i64) -> Self {
        let value = self.v.int(val);
        self.add(id, value)
    }
    pub fn i(mut self, id: &'a str, val: i64) -> Self {
        let value = self.v.i(val);
        self.add(id, value)
    }

    pub fn flt(mut self, id: &'a str, val: f64) -> Self {
        let value = self.v.flt(val);
        self.add(id, value)
    }
    pub fn f(mut self, id: &'a str, val: f64) -> Self {
        let value = self.v.f(val);
        self.add(id, value)
    }

    pub fn bool(mut self, id: &'a str, val: bool) -> Self {
        let value = self.v.bool(val);
        self.add(id, value)
    }
    pub fn b(mut self, key: &'a str, val: bool) -> Self {
        let value = self.v.b(val);
        self.add(key, value)
    }
    pub fn y(mut self, id: &'a str) -> Self {
        let val = self.v.y();
        self.add(id, val)
    }
    pub fn n(mut self, id: &'a str) -> Self {
        let val = self.v.n();
        self.add(id, val)
    }

    pub fn nul(mut self, id: &'a str) -> Self {
        let val = self.v.nul();
        self.add(id, val)
    }
    pub fn nil(mut self, id: &'a str) -> Self {
        let val = self.v.nil();
        self.add(id, val)
    }
}
impl<'a> Clone for KdlPropertiesBuilder<'a> {
    fn clone(&self) -> Self {
        Self {
            props: &mut self.props.clone(),
            v: &mut self.v.clone(),
        }
    }
}
impl<'a> Default for KdlPropertiesBuilder<'a> {
    fn default() -> Self {
        Self {
            props: &mut HashMap::new(),
            v: &mut KdlValueBuilder,
        }
    }
}
// impl<'a> Default for KdlValueBuilder {
//     fn default() -> Self {
//         Self {
//             v: &mut KdlValue::default(),
//         }
//     }
// }
impl<'a> Clone for KdlNodeBuilder<'a> {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            v: self.v.vals.clone(),
            props: self.p.clone(),
            c: self.c.clone()
        }
    }
}

impl<'a> KdlNodeBuilder<'a> {
    pub fn new(name: &'a mut str) -> Self {
        Self {
            n: name,
            v: &mut KdlValuesBuilder::new(),
            p: &mut KdlPropertiesBuilder::new(),
            c: &mut Vec::new(),
        }
    }
    pub fn build(mut self) -> KdlNode {
        KdlNode {
            name: self.n.to_string(),
            values: self.v.build(),
            properties: self.p.build(),
            children: self.c.into_iter().map(|c| c.build()).collect(),
        }
    }
    // pub fn reset(mut self) -> Self {
    //     self.reset_children().reset_values().reset_properties()
    // }
    pub fn extend(mut self, val: &'a mut KdlNodeBuilder) -> Self {
        self.v = &mut self.v.extend(&mut val.v);
        self.p = &mut self.p.extend(&mut val.p);
        self.c.extend(val.c.to_owned());
        self
    }
    pub fn extend_children(mut self, val: &'a mut KdlNodeBuilder) -> Self {
        let output = Vec::new();
        for child in self.c..iter() {
            output.push();
        }
        self.c.extend(output);
        self
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.n = name;
        self
    }
    pub fn reset_values(mut self) -> Self {
        self.v = &mut self.v.reset();
        self
    }
    pub fn reset_properties(mut self) -> Self {
        self.p = &mut self.p.reset();
        self
    }

    // pub fn reset_child(mut self, child: usize) -> Self {
    //     self.c[child] = &mut self.c[child].reset();
    //     self
    // }
    pub fn reset_child_values(mut self, child: usize) -> Self {
        self.c[child] = &mut self.c[child].clone().reset_values();
        self
    }
    pub fn reset_child_properties(mut self, child: usize) -> Self {
        self.c[child] = &mut self.c[child].clone().reset_properties();
        self
    }
    // pub fn reset_children(mut self) -> Self {
    //     // run reset on each child node
    //     self.c = self
    //         .c
    //         .iter()
    //         .map(|c| -> &KdlNodeBuilder { &c.reset() })
    //         .collect();
    //     self
    // }
    // pub fn reset_children_values(mut self) -> Self {
    //     // run reset on each child node
    //     self.c = self.c.into_iter().map(|c| c.reset()).collect();
    //     self
    // }
    // pub fn reset_children_properties(mut self) -> Self {
    //     // run reset on each child node
    //     self.c = self.c.into_iter().map(|c| c.reset()).collect();
    //     self
    // }
    pub fn add(mut self, val: &'a mut KdlValue) -> Self {
        self.v = &mut self.v.add(val);
        self
    }
    pub fn val(mut self, val: &'a mut KdlValue) -> Self {
        self.v = &mut self.v.add(val);
        self
    }
    pub fn rem(mut self, index: usize) -> Self {
        self.v = &mut self.v.rem(index);
        self
    }
    pub fn join(mut self, vals: &'a mut Vec<&'a mut KdlValue>) -> Self {
        self.v = &mut self.v.join(vals);
        self
    }
    pub fn vals(mut self, vals: &'a mut Vec<&'a mut KdlValue>) -> Self {
        for val in vals {
            self.v = &mut self.v.add(val);
        }
        self
    }
    pub fn value(mut self, index: usize, val: &'a KdlValue) -> Self {
        self.v = &mut self.v.rem(index);
        self.v = &mut self.v.add(val);
        self
    }
    pub fn set(mut self, vals: &'a mut Vec<&'a mut KdlValue>) -> Self {
        self.v = &mut self.v.set(vals);
        self
    }
    pub fn values(mut self, vals: &'a mut Vec<&'a mut KdlValue>) -> Self {
        self.v.vals = vals;
        self
    }

    pub fn put(mut self, key: &'a str, val: &'a KdlValue) -> Self {
        self.p = &mut self.p.add(key, &val);
        self
    }
    pub fn prop(mut self, key: &'a str, val: &'a KdlValue) -> Self {
        self.p = &mut self.p.add(key, &val);
        self
    }
    pub fn props(mut self, props: &'a HashMap<&'a str, &'a KdlValue>) -> Self {
        for (key, val) in props {
            self.p = &mut self.p.add(key, &val);
        }
        self
    }
    pub fn property(mut self, key: &'a str, val: &'a KdlValue) -> Self {
        self.p = &mut self.p.rem(key);
        self.p = &mut self.p.add(key, val);
        self
    }
    pub fn properties(mut self, props: &'a mut HashMap<&'a str, &'a KdlValue>) -> Self {
        self.p.props = props;
        self
    }

    pub fn child(mut self, child: &'a mut KdlNodeBuilder<'a>) -> Self {
        self.c.push(&mut child);
        self
    }
    pub fn children(mut self, children: &'a mut Vec<KdlNodeBuilder<'a>>) -> Self {
        self.c.extend(children);
        self
    }
    pub fn set_child(mut self, index: usize, child: &'a mut KdlNodeBuilder<'a>) -> Self {
        self.c.remove(index);
        self.c.insert(index, child);
        self
    }
    pub fn set_children(mut self, children: &'a mut Vec<&'a mut KdlNodeBuilder<'a>>) -> Self {
        self.c = children;
        self
    }
    pub fn remove_child(mut self, child: usize) -> Self {
        self.c.remove(child);
        self
    }
    pub fn remove_children(mut self) -> Self {
        self.c.clear();
        self
    }
    pub fn str(mut self, val: &'a str) -> Self {
        self.v = &mut self.v.str(val);
        self
    }
    pub fn s(mut self, val: &'a str) -> Self {
        self.v = &mut self.v.s(val);
        self
    }

    pub fn int(mut self, val: i64) -> Self {
        self.v = &mut self.v.int(val);
        self
    }
    pub fn i(mut self, val: i64) -> Self {
        self.v = &mut self.v.i(val);
        self
    }

    pub fn flt(mut self, val: f64) -> Self {
        self.v = &mut self.v.flt(val);
        self
    }
    pub fn f(mut self, val: f64) -> Self {
        self.v = &mut self.v.f(val);
        self
    }

    pub fn bool(mut self, val: bool) -> Self {
        self.v = &mut self.v.bool(val);
        self
    }
    pub fn b(mut self, val: bool) -> Self {
        self.v = &mut self.v.b(val);
        self
    }
    pub fn y(mut self) -> Self {
        self.v = &mut self.v.y();
        self
    }
    pub fn n(mut self) -> Self {
        self.v = &mut self.v.n();
        self
    }

    pub fn null(mut self) -> Self {
        self.v = &mut self.v.nul();
        self
    }
    pub fn nul(mut self) -> Self {
        self.v = &mut self.v.nul();
        self
    }
    pub fn nil(mut self) -> Self {
        self.v = &mut self.v.nil();
        self
    }
}
