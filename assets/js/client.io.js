/* EVENT HANDLER */
function addEvent(element, event, handler) {
	element.addEventListener ? element.addEventListener(event, handler, !1) : element.attachEvent && element.attachEvent("on" + event, handler)
}

var rs = !1, l = "vi", s = document.createElement('script');
s.type = "text/javascript";
s.src = "/socket.io/socket.io.js";
s.async = true;
s.onload = s.onreadystatechange = function() {
	if (!rs && (!this.readyState || this.readyState == 'complete')) {
		var server = "http://localhost:3000/chat";
		var socket = io.connect(server);
		socket.on('hello', function (data) {
			console.log(data);
			var d = {url : document.URL, cid : "tisuu", pmid : 0, lzmid : 0, lang : l};
			socket.emit((rs ? "hallo" : "hello"), d);
			rs = !0;
		});
		socket.on('message', function (data) {
			console.log(data);
			if (data.type == 2) {
				$('#conversation').append('<b>'+data.brain + ':</b> ' + data.msg + '<br>');
			} else if (data.type == 1) {
				$('#conversation').append('<b>You:</b> ' + data.msg + '<br>');
			} else if (data.type == 0) {
				$('#conversation').append('<b>Welcome:</b> ' + data.msg + '<br>');
			} else {
				console.log('Unknow type!!');
			}
		});
		// on load of page
		$(function() {
			// when the client clicks send
			$("#btnSend").click(function () {
				var msg = $("#chatbox").val();
				$("#chatbox").val("");
				// prepare message to send
				var d = {url : document.URL, cid : "tisuu", rmsg : msg, pmid : 1, lzmid : 1, lang : l};
				socket.emit("message", d);
			});
			$("#chatbox").keypress(function(e) {
				if (e.which == 13) {
					// $(this).blur();
					$("#btnSend").click();
				}
			});
		});
	}
};
document.body.appendChild(s);
