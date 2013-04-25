/* EVENT HANDLER */
function addEvent(element, event, handler) {
	element.addEventListener ? element.addEventListener(event, handler, !1) : element.attachEvent && element.attachEvent("on" + event, handler)
}

function addLoadEvent(func) {
	var oldonload = window.onload;
	if (typeof window.onload != 'function' ) {
		window.onload = func;
	}
	else {
		window.onload = function() {
			oldonload();
			func();
		}
	}
}

function shiftUp(dialogbox, max) {
	var dbox = document.getElementById(dialogbox);
	if(dbox.hasChildNodes()) {
		for(; max < dbox.childNodes.length;) {
			dbox.removeChild(dbox.firstChild);
		}
	}
}

addLoadEvent(function() {
	var txtUsername = "#txtUserName";
	var nickname = $(txtUsername).val() || "You";
	var msgid = "#txtMessage";
	var logid = "#txtDialog";
	var dialogbox = "txtDialog";
	var sendid = "#btnSend";
	var joinid = "#btnJoin";
	var cid = "alice";
	var lang = "vi";
	var max = 6;
	var rs = new RiveScript({debug: false});
	// Load our files from the brain/ folder.
	rs.loadFile([
	"ai/brain/eliza/begin.rs",
	"ai/brain/eliza/admin.rs",
	"ai/brain/eliza/clients.rs",
	"ai/brain/eliza/eliza.rs",
	"ai/brain/eliza/myself.rs",
	"ai/brain/eliza/rpg.rs"
	], function() {
		$(msgid).removeAttr("disabled").focus();
		rs.sortReplies();
	}, function(err) {
		console.log("Loading error: " + err);
	});


	$(function() {
		$(joinid).bind( "click", function(event, ui) {
			nickname = $(txtUsername).val() || "You";
			cid = $('input:radio[name=chkAlice]:checked').val();
			return true;
		});
		$(sendid).click(function () {
			var msg = $(msgid).val();
			$(msgid).val("");
			$(logid).append("<div><span class='user'>" + nickname + ":</span> " + msg + "</div>");
			var reply = rs.reply("demo", msg);
			reply = reply.replace(/\n/g, "<br>");
			$(logid).append("<div><span class='bot'>" + cid + ":</span> " + reply + "</div>");
			shiftUp(dialogbox, max);
			return false;
		});
		$(msgid).keypress(function(e) {
			if (e.which == 13) {
				$(sendid).click();
				return false;
			}
		});
	});
});