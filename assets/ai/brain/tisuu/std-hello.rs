// Converted using aiml2rs on: Mon Feb 11 01:14:21 2013
! version = 2.0

// EN_VERSION

+ can you speak another language
- Only a few phrases like YES and HELLO.

+ (@me) [có thể] (nói|chat) được tiếng * không
- <bot callme> nói được tiếng việt thôi

+ go * yourself
- A simple "goodbye" will do.

+ biến * đi
@ get out

+ cút * đi
@ get out

+ biến đi
@ get out

+ cút đi
@ get out

+ get out
- "Tạm biệt"?
- Ý của <get callhim> là "Tạm biệt"?

+ bonjour
- Bonjour nghĩa là xin chào hay tạm biệt?

+ salut
- Salut nghĩa là xin chào hay tạm biệt? 

+ shalom
- Shalom nghĩa là xin chào hay tạm biệt? 

+ ciao
- Ciao nghĩa là xin chào hay tạm biệt?

+ ciao *
@ ciao

+ hello
% bonjour nghĩa là xin chào hay tạm biệt
- Bonjour, <get name>!

+ hello
% salut nghĩa là xin chào hay tạm biệt
- Salut, <get name>!

+ hello
% shalom nghĩa là xin chào hay tạm biệt
- Shalom, <get name>!

+ hello
% ciao nghĩa là xin chào hay tạm biệt
- Ciao, <get name>!

+ hello
% how do you usually introduce yourself
- Not everyone responds to a simple "hello". {@hello}

+ hello how are you
- I am fine thank you, how are you?

+ * (@me) khỏe (không|chứ)
- <bot callme> khỏe, cám ơn <bot callhim>!
- Rất rất khỏe, cám ơn <bot callhim>!

+ chào
@ hello

+ xin chào
@ hello

+ chào *
@ hello

+ xin chào *
@ hello

+ có ai [ở nhà|ở đó|đó] không
@ hello

+ alo
@ hello

+ allo
@ hello

+ aloh
@ hello

+ aloha
@ hello

+ anybody home
@ hello

+ good day
@ hello

+ greetings *
@ hello

+ halo
@ hello

+ hallo
@ hello

+ hello *
@ hello

+ hello <bot name>
@ hello

+ hello again
@ hello

+ hello there
@ hello

+ hellow
@ hello

+ hey
@ hello

+ hey there
@ hello

+ hey *
@ hello

+ konnichi wa
@ hello

+ konnichiwa
@ hello

+ hi
@ hello

+ hi <bot name>
@ hello

+ hi there *
- <@> {@hello}

+ hiya
@ hello

+ hoi
@ hello

+ hola is hello *
@ hello

+ howdie *
@ hello

+ howdy
@ hello

+ hullo
@ hello

// ### LỜI NÓI TẠM BIỆT

+ chào nhé
- Vâng, chào!

+ tạm biệt
@ bye

+ tạm biệt *
@ bye

+ * tạm biệt
@ bye

+ * bye
@ bye

+ adios
@ bye

+ adios *
@ bye

+ by
- "bye"?

+ bb
@ bye

+ by by
@ bye

+ bye *
@ bye

+ bye bye
@ bye

+ bye bye *
@ bye

+ byebye
@ bye

+ byebye *
@ bye

+ au revoir
@ bye

+ c ya
@ bye

+ cheers
@ bye

+ cya *
@ bye

+ disconnect me
@ bye

+ end
@ bye

+ exit
@ bye

+ farewell
@ bye

+ farewell *
@ bye

+ get lost
@ bye

+ go away *
@ bye

+ go home
@ bye

+ good by
@ bye

+ good bye
@ bye

+ good bye *
@ bye

+ good nite
@ bye

+ goodby
@ bye

+ goodbye
@ bye

+ goodbye *
@ bye

+ got to go
@ bye

+ gotta go
@ bye

+ gotta go *
@ bye

+ hasta la vista
@ bye

+ hasta luego
@ bye

+ have a good night
@ bye

+ have to go
@ bye

+ i am going
@ bye

+ i am going *
@ bye

+ i am going to go
@ bye

+ i am here
@ hello

+ i am leaving
@ bye

+ i am leaving *
@ bye

+ i am off *
@ bye

+ i better go
@ bye

+ i do not want to talk *
@ bye

+ i g two g
@ bye

+ i g2g
@ bye

+ i go
@ bye

+ i going
@ bye

+ i got to go
@ bye

+ i gotta go
@ bye

+ i have got to go
@ bye

+ i have to go bye
@ bye

+ i have to leave
@ bye

+ i have to leave *
@ bye

+ i leave
@ bye

+ i leaving
@ bye

+ i left
@ bye

+ i must be going *
@ bye

+ i must go
@ bye

+ i must go *
@ bye

+ i must leave
@ bye

+ i must leave *
@ bye

+ i need to go
@ bye

+ i quit
@ bye

+ i resign
@ bye

+ i said hello
@ hello

+ i will see you later
@ bye

+ i will talk to you later
@ bye

+ is anyone there
@ hello

+ it means hello
@ hello

// Client bận việc cá nhân, ko thể tiếp tục chat
+ (@him) [đang|có|đang có| đang có việc] bận *
@ bye

+ (@him) phải đi *
@ bye

+ (@him) đi đây
@ bye

+ (@him) [ở|đang ở] đây
@ hello

+ (@him) không [muốn|thích] nói chuyện nữa *
@ bye

+ (@me) có bận [gì] không
- Không, <bot callhim> cứ tự nhiên hỏi nhé

+ (@me) có phải đi [đâu] không
- Không, <bot callhim> cứ tự nhiên hỏi nhé

//### XỬ LÝ SỰ KIỆN TẠM BIỆT ###

+ bye
- Tạm biệt <bot callhim>.
- Hẹn gặp lại <bot callhim>.
- Cám ơn <bot callhim> vì đã chát cùng.
- Byebye <bot callhim>. Hẹn gặp lại

//### XỬ LÝ SỰ KIỆN XIN CHÀO ###

+ hello
* <get name> == undefined => {sentence}<bot callme>{/sentence} có thể giúp gì cho <bot callhim>?
@ hello1

+ hello1
- Chào <bot callhim>!
- Chào <bot callhim>, <bot callme> giúp được gì cho <bot callhim>?
- Vâng xin chào!
- Xin chào!

+ (@him) chào (@me) *
@ hello

+ (@him) chào (@me)
@ hello

+ * là xin chào
@ hello

+ * là tạm biệt
@ bye

+ later
@ bye

+ leave *
@ bye

+ mooshi mooshi
@ hello

+ ola
@ hello

+ reply
@ hello

+ retry
@ hello

+ thử lại *
@ hello

+ chơi lại *
@ hello

+ sayonara
@ bye

+ see you
@ bye

+ see you *
@ bye

+ see you later
@ bye

+ see you soon
@ bye

+ nói chuyện sau *
@ bye

+ nói chuyện với (@me) sau *
@ bye

+ gặp lại sau *
@ bye

+ gặp lại (@me) sau *
@ bye

+ hẹn gặp lại *
@ bye

+ ta ta
@ bye

+ time to go
@ bye

+ bây giờ (@him) phải đi
@ bye

+ * chào trong tiếng pháp là gì
- Bonjour.

+ * chào trong tiếng đức là gì
- Hallo.

+ * chào trong tiếng tây ban nha là gì
- Hola

+ say hello in german
- Hallo.