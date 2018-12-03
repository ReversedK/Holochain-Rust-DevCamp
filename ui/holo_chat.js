var activeRoom;
var active_room_name = 'general';
var call_to_api;
let initialized=false;
let myConfig = {};

function getConfiguration() {
  var configuration = {
    url: "ws://localhost:3400/",
    agent: "chat",
    instance: "test-instance",
    dna: "Qm328wyq38924y"
  };

  return configuration;
}

function init() {
  if (initialized) return active_room_name="general";
  console.log('--------------------------------init----------------------')
  var configuration = getConfiguration();
  // create my profile
  call_to_api(configuration.instance, "chat", "main", "add_profile")({
    name: "Ba",
    country: "France"
  }).then(response => {
    console.log("profile addr received");
    response = JSON.parse(response);
    myConfig.user = { profile_addr: response };
  });

  // create main channel
  const NewChannelParams = {
    name: "general",
    description: "general chatroom",
    public: true
  };

  call_to_api(configuration.instance, "chat", "main", "create_channel")(NewChannelParams).then(response => {
    console.log("-------------------", response,myConfig);
   
    response = JSON.parse(response);
    active_room_name = NewChannelParams.name;
    initialized=true;
   /* setInterval(function() {
       getMessages();
        getRooms();
    }, 3000);*/
  });

  // populate channel list

  // populate profile list
}

function getRooms() {
  if(!initialized) return;
  var configuration = getConfiguration();
  call_to_api(configuration.instance, "chat", "main", "get_channel_list")(
    {}
  ).then(response => {
    console.log(response);
    var rooms = JSON.parse(response);
    $("#rooms").empty();
    rooms = rooms.sort(function(a, b) {
      if (a.entry.name < b.entry.name) return -1;
      if (a.entry.name > b.entry.name) return 1;
      return 0;
    });
    for (i = 0; i < rooms.length; i++) {
      $("#rooms").append(
        `<div  data-id="` +rooms[i].address +`" data-name="` +rooms[i].entry.name +`" class="chat_list" >
        <div class="chat_people">           
            <div class="chat_ib">
                <h5>`+rooms[i].entry.name+`</h5>
                <p>`+rooms[i].entry.description+`</p>
            </div>
        </div>
    </div>`
      );
    }
    $("#rooms div.chat_list").click(selectRoom);
    setActiveRoom();
  });
}

function addRoom() {
  var configuration = getConfiguration();
  var room = {
    name: $("#room-name-input").val(),
    access: "public"
  };

  $("#room-name-input").val("");

  call_to_api(configuration.instance, "chat", "main", "create_channel")({
    name: room.name,
    description: "user generated room",
    public: room.public == "public" ? true : false
  }).then(response => {
    console.log("Response is :" + response);
  });
}

function selectRoom(event) {
  console.log($(this).data("name"));
  $("#rooms div").removeClass("selected-room active_chat");
  activeRoom = $(this).data("id");
  active_room_name = $(this).data("name");
  setActiveRoom();
}

function setActiveRoom() {
  var roomElement = $("#rooms div[data-id=" + activeRoom + "]");
  $(roomElement).addClass("selected-room active_chat");
  $("#messages-header").text("Messages in #" + active_room_name);
  getMessages();
}

function getMessages() {
  var configuration = getConfiguration();
  let payload = {
    channel_name: active_room_name || ""
  };
  if(typeof active_room_name=="undefined") return;
  console.log("getting messages for "+active_room_name);
  call_to_api(configuration.instance, "chat", "main", "get_messages")(
    payload
  ).then(response => {
    var messages = JSON.parse(response);
    console.log("messages:",messages);
    if (messages.error) return;

    var sorted_messages = messages.sort(function(a, b) {
      return new Date(b.timestamp) - new Date(a.timestamp);
    });

    $("#messages").empty();

    for (var i = 0; i < sorted_messages.length; i++) {  
      if(sorted_messages[i].from==myConfig.user.profile_addr) {
        $("#messages").append(
          `<div class="outgoing_msg">
          <div class="sent_msg"><p>
              ` +
            sorted_messages[i].text +
            `</p>
                  <span class="time_date">` +
            sorted_messages[i].timestamp +
            `</span></div>
          </div>`);
      }  else  $("#messages").append(
        `<div class="incoming_msg">
        <div class="incoming_msg_img"> <img src="https://ptetutorials.com/images/user-profile.png" alt="sunil"> </div>
        <div class="received_msg">
            <div class="received_withd_msg">
                <p>` +
          sorted_messages[i].text +
          `</p>
                <span class="time_date">` +
          sorted_messages[i].timestamp +
          `</span></div>
        </div>
    </div>
    </div>`
      );
    }
  });
}

function sendMessage() {
  var text = $("#message-input").val();
  var configuration = getConfiguration();
  console.log("sending message");
  var payload = {
    message: { text, timestamp: new Date(),from:myConfig.user.profile_addr },
    channel_name: active_room_name || "general"
  };$("#message-input").val("");
  call_to_api(configuration.instance, "chat", "main", "post_message")(payload).then(response => {
    console.log("response : +" + response);    
  });
}

$(window).ready(function() {
  var configuration = getConfiguration();
  holoclient.connect(configuration.url).then(({ call, close }) => {
    call_to_api = call;
    init();
  });

  $("#room-name-button").click(addRoom);

 

  $("#message-button").click(sendMessage);
  $("#room-name-input").keyup(function(event) {
    if (event.keyCode == 13) $("#room-name-button").click();
  });

  $("#message-input").keyup(function(event) {
    if (event.keyCode == 13) $("#message-button").click();
  });
  setInterval(function() {
    getMessages();
     getRooms();
 }, 3000);

  
});
