"use strict";
const definitions = require("../../lib/definitions");

module.exports = function generateConstants() {
  let output = `
/*
 * This file is auto-generated! Do not modify it directly.
 * To re-generate run 'make build'
 */

use std::collections::HashMap;
use lazy_static::lazy_static;

fn to_hashmap(arr: Vec<(&str, Vec<&str>)>) -> HashMap<String, Vec<String>> {
    arr.iter()
        .map(|(k, v)| (k.to_string(), v.iter().map(|i| i.to_string()).collect()))
        .collect()
}

lazy_static!{
    // TODO: consider storing them precomputed using a PHF.
    // TODO: empty array is being converted to vec![""]. Turns out not to matter
    // for the moment, but best to fix it eventually.
    pub static ref ALIAS_KEYS: HashMap<String, Vec<String>> = to_hashmap(vec![
`;

  Object.keys(definitions.ALIAS_KEYS).forEach(type => {
    output += `        ("${type}", vec!["${definitions.ALIAS_KEYS[type].join(
      '", "'
    )}"]),\n`;
  });

  output += `]);

pub static ref FLIPPED_ALIAS_KEYS: HashMap<String, Vec<String>> = to_hashmap(vec![
`;

  Object.keys(definitions.FLIPPED_ALIAS_KEYS).forEach(type => {
    output += `        ("${type}", vec!["${definitions.FLIPPED_ALIAS_KEYS[
      type
    ].join('", "')}"]),\n`;
  });

  output += `    ]);
}
`;
  return output;
};
