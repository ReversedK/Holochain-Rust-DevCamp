// This test file uses the tape testing framework. 
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")
const app2 = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

const testNewChannelParams = {
  name: "test new channel",
  description: "for testing...",
  public: true
}

const testMessage = {
  timestamp: "100000",
  text : "Some text"
}

test('Can create a public channel with no other members and retrieve it', (t) => {
  const create_result = app.call('chat', 'main', 'create_channel', testNewChannelParams)
  console.log(create_result)
  t.deepEqual(create_result.address.length, 46)

  const get_result = app.call('chat', 'main', 'get_my_channels', {})
  console.log(get_result)
  t.deepEqual(get_result.length, 1)

  t.end()
})
/*
test('Can post a message to the channel and retrieve', (t) => {
  const create_result = app.call('chat', 'main', 'create_channel', testNewChannelParams)
  console.log(create_result)
  const channel_addr = create_result.address
  t.deepEqual(channel_addr.length, 46)

  const get_result = app.call('chat', 'main', 'get_my_channels', {})
  console.log(get_result)
  t.deepEqual(get_result.length, 1)

  const post_result = app.call('chat', 'main', 'post_message', {channel_name: testNewChannelParams.name, message: testMessage})
  console.log(post_result)
  t.deepEqual(post_result, {Ok: { success: true}})

  const get_message_result = app.call('chat', 'main', 'get_messages', {channel_name: testNewChannelParams.name, min_count: 10})
  console.log(get_message_result)
  t.deepEqual(get_message_result[0], testMessage)
  t.end()
})*/

test('create a profile', (t) => {
  const result = app.call("chat", "main", "add_profile", { name: "Ba" ,country:"France"})
  console.log(result)
  t.equal(result.error, undefined)
  t.end()
})

test('Get my profile', (t) => {
  const result = app.call("chat", "main", "get_my_profile", {})
  console.log(result)
  t.notEqual(result.name, "Ba")
  t.end()
})
let r;
test('Get profile List', (t) => {
  const result = app.call("chat", "main", "get_profile_list",{})
  console.log(result);
  r = result
  t.equal(result.error, undefined)
  t.end()
})

test('Get A profile by address', (t) => {
  const result = app.call("chat", "main", "get_a_profile",{addr:r[0].address})
  console.log(result)
  t.equal(result.error, undefined)
  t.end()
})



test('Get channel List', (t) => {
  const result = app.call("chat", "main", "get_channel_list",{})
  console.log(result);
  r = result
  t.equal(result.error, undefined)
  t.end()
})

