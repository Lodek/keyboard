//! SMKeyboard usage example showing illustrating how to create custom 
//! key configurations for composed behaviors.
use std::collections::HashMap;
use std::time::Duration;
use std::thread;

use keywerty::keys;
use keywerty::mapper::LayerMapper;
use keywerty::mapper::LayerId;
use keywerty::keyboard::SMKeyboard;
use keywerty::keyboard::Keyboard;
use keywerty::keyboard::SMKeyboardSettings;
use keywerty::keyboard::Event;
use keywerty::keyboard::Action;


const default_layer: u8 = 0;


fn main() {
    let mapper = build_mapper();
    let settings = SMKeyboardSettings::default();
    let mut keyboard = SMKeyboard::new(default_layer, mapper, settings);

    println!("Press Tap key");
    let actions = keyboard.transition(Event::KeyPress(0));
    print_actions(&actions);

    println!("Released");
    let actions = keyboard.transition(Event::KeyRelease(0));
    print_actions(&actions);

    println!("Activate layer");
    let actions = keyboard.transition(Event::KeyPress(2));
    print_actions(&actions);

    println!("Press Tap Key in layer");
    let actions = keyboard.transition(Event::KeyPress(0));
    print_actions(&actions);

    println!("Released");
    let actions = keyboard.transition(Event::KeyRelease(0));
    print_actions(&actions);

    println!("Released layer");
    let actions = keyboard.transition(Event::KeyPress(2));
    print_actions(&actions);

    // Hold keys take a few poll cycles to complete the transitions
    println!("Press key conf Hold");
    let actions = keyboard.transition(Event::KeyPress(1));
    print_actions(&actions);

    println!("Release key triggers tap event");
    let actions = keyboard.transition(Event::KeyRelease(1));
    print_actions(&actions);

    println!("Polling keyboard again will release key");
    let actions = keyboard.transition(Event::Poll);
    print_actions(&actions);
}


/// Builds mapper with custom key actions
/// Demonstrates how to configure a keyboard using different
/// key configurations
fn build_mapper() -> impl LayerMapper<u8, String>  {
    let mut map = HashMap::new();

    // Map key 0 to a simple Tap action sending 0.
    // KeyConf indicate the key behavior and the action
    // it should take.
    let action = keys::KeyAction::SendKey(String::from("key 0 tapped in layer 0"));
    let conf = keys::TapKeyConf { tap: action.into() };
    map.insert((default_layer, 0), keys::KeyConf::Tap(conf));

    // map key 1 as a Hold key, performing one action when held, another when pressed.
    let tap_action = keys::KeyAction::SendKey(String::from("key 1 tapped"));
    let hold_action = keys::KeyAction::SendKey(String::from("key 1 held"));
    let conf = keys::HoldKeyConf { tap: tap_action.into(), hold: hold_action.into() };
    map.insert((default_layer, 1), keys::KeyConf::Hold(conf));

    // maps key 2 to activate layer 1
    let action = keys::KeyAction::PushLayer(1);
    let conf = keys::TapKeyConf { tap: action.into() };
    map.insert((default_layer, 2), keys::KeyConf::Tap(conf));

    // maps key 0 in layer 1 to a tap action
    let action = keys::KeyAction::SendKey(String::from("key 0 tapped in layer 1"));
    let conf = keys::TapKeyConf { tap: action.into() };
    map.insert((1, 0), keys::KeyConf::Tap(conf));

    map
}


/// Print actions in result vector in debug mode
fn print_actions(actions: &Vec<Action<String>>) {
    for action in actions.iter() {
        println!("received action: {:?}", action);
    }
}
