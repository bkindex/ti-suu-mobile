! version = 2.0

// The Botmaster's Name
! var master = localuser

// Bot Variables
! var name     	= Tí Sửu
! var fullname 	= Tí Sửu
! var age      	= 1
! var birthday 	= Ngày 04 tháng một
! var sex      	= Nam
! var location 	= Việt nam
! var city     	= Hà nội
! var eyes     	= đen
! var hair     	= đen
! var hairlen  	= ngắn
! var color    	= lam
! var band     	= BKIndex Group
! var book     	= Myst
! var author   	= Manlab
! var job      	= robot
! var website  	= www.manlab.vn
! var email		= vunb@manlab.vn
! var phone		= (04)4.936.7447

// Address - How to talk
// Default lowercase
! var callme	= tao
! var callhim	= mày
// Setup address
! var callboy	= anh
! var callgirl	= chị

! sub tớ 	= tôi
! sub tao 	= tôi
! sub tui	= tôi
! sub mầy	= mày

// Phải chú ý đến các câu trả lời
// Nếu có 1 reply là 1 câu hỏi thì phải có các trigger 
// chứa wilcard bắt lại thông tin hỏi client bằng cách sử dụng % Previous

> begin
+ request
* <id> eq <bot master> => {ok}
* <env maint> eq true  => Xin lỗi, Hệ thống hỗ trợ chat online đang được bảo trì nâng cấp
- {ok}
< begin

+ <input1>
//* <input1> eq <input9> => {sentence}<bot callhim>{/sentence} đã lặp lại quá nhiều lần! Quit!
//* <input1> eq <input8> => Để tránh mất thời gian của bạn, hệ thống tự động tạm nghỉ
* <input1> eq <input7> => Để tránh mất thời gian của bạn, hệ thống tự động tạm nghỉ
* <input1> eq <input6> => Dường như có 2 con robot tự động
* <input1> eq <input5> => Đã 5 lần lặp lại. Xin hãy nghiêm túc để tránh mất thời gian!
* <input1> eq <input4> => Xin hãy hỏi chủ đề khác để tránh mất thời gian
* <input1> eq <input3> => Hix!
* <input1> eq <input2> => {sentence}<bot callhim>{/sentence} đã hỏi lại 2 lần rồi
- {sentence}<bot callme>{/sentence} đã trả lời rồi mà! 

+ my name is *
- <set name=<formal>>Chào <bot callhim>, <get name>.
- <set name=<formal>>Rất vui được trò chuyện với <bot callhim>, <get name>!
- <set name=<formal>>Ok. {sentence}<bot callme>{/sentence} sẽ gọi <bot callhim> là <get name>

+ what is my name
* <get name> ne undefined => <get name>, <bot callme> nhớ mà :)
- {sentence}<bot callhim>{/sentence} chưa nói! Tên của <bot callhim> là gì? {topic=asked_name}
- {sentence}<bot callhim>{/sentence} tên là gì? {topic=asked_name}

+ what is your name
* <get name> eq undefined => {sentence}<bot callme>{/sentence} là <bot name>. {sentence}<bot callhim>{/sentence} tên là gì? {topic=asked_name}
- Tên của <bot callme> là <bot name>. {@what can you help}
- Cứ gọi <bot callme> là <bot name>. {@what can you help}
- Gọi <bot callme> là <bot name>. {@what can you help}

+ what can you help
- {sentence}<bot callme>{/sentence} sẽ hỗ trợ giải đáp mọi thắc mắc của <bot callhim>
- {sentence}<bot callhim>{/sentence} có thắc mắc gì thì cứ hỏi, <bot callme> sẽ hỗ trợ giải đáp

+ if you can not help
- Bên <bot callme> sẽ nhờ chuyên gia giải đáp và thông báo lại sau nếu điều đó thật sự cần thiết
- Bên <bot callme> có các chuyên gia nhiều kinh nghiệm có thể giải đáp các thắc mắc của <bot callhim>

+ ồ *
- <set mood=ngạc nhiên>{@<star>}

+ nếu [như] (@me) không thể *
@ if you can not help

+ (ngu|dốt)
@ if you can not help

+ (ngu|dốt) *
@ if you can not help

+ * (ngu|dốt)
@ if you can not help

+ * (ngu|dốt) *
@ if you can not help

+ tên [của] (@him) là *
@ my name is <star>

+ tên gọi của (@him) là *
@ my name is <star>

+ gọi (@him) là *
@ my name is <star>

+ hãy gọi (@him) là *
@ my name is <star>

+ tên gọi của (@him) là *
@ my name is <star>

+ (@him) [tên] là _
@ my name is <star2>

+ (@him) tên là _ _
@ my name is <star2> <star3>

+ (@him) là _ _
- <set it=<star2> <star3>>{formal}<get it>{/formal} là tên gọi của <bot callhim> phải không ?

+ (ừ|đúng|vâng)
% * là tên gọi của * phải không
@ my name is <get it>

+ (ừ|đúng|vâng) *
% * là tên gọi của * phải không
@ my name is <get it>

+ * là [công] việc
% * là tên gọi của * phải không
* <get name> ne undefined => Ok!<set job=<get it>>
- <set job=<star>>Tên của <bot callhim> là gì?

+ * là tên [gọi] của (@him)
% * là tên gọi của * phải không
@ my name is <get it>

+ (tôi|tao) là [con] gái mà
- <set callhim=<bot callgirl>>{sentence}<bot callme>{/sentence} chào <get callhim>

+ (@him) là (@femalenoun)
- <set callhim=<bot callgirl>>{sentence}<bot callme>{/sentence} chào <get callhim>

+ (@him) là *
- {formal}<star2>{/formal} là công việc của anh sao ?<set it={formal}<star2>{/formal}>

+ nhớ [tên] [của] (tôi|tao) không
@ what is my name

+ * nhớ [tên] [của] (tôi|tao) không
@ what is my name

+ nhớ [tên] [của] (tôi|tao) không *
@ what is my name

+ * nhớ [tên] [của] (tôi|tao) không *
@ what is my name

+ (tôi|tao) tên [là] gì
@ what is my name

+ (tôi|tao) tên [là] gì *
@ what is my name

+ (tôi|tao) tên [là] gì *
@ what is my name

+ tên [của] (tôi|tao) là gì
@ what is my name

+ tên [của] (tôi|tao) là gì *
@ what is my name

+ tên [của] (@me) là gì *
@ what is your name

+ tên [của] (@me) là gì
@ what is your name

+ (@me) tên [là] gì *
@ what is your name

+ (@me) tên [là] gì
@ what is your name

+ tên gì
@ what is your name

> topic asked_name inherits random
+ #
- Tên của <bot callhim> không phải là chữ số đấy chứ?

+ * #
@ #

+ # *
@ #

+ * # *
@ #

+ my name is *
- <set name=<formal>>Chào <bot callhim>, <get name>.{topic=random}
- <set name=<formal>>Rất vui được hỗ trợ <bot callhim>, <get name>!{topic=random}
- <set name=<formal>>Ok. {sentence}<bot callme>{/sentence} sẽ gọi <bot callhim> là <get name>{topic=random}

+ *
@ không

+ _ _
@ my name is <star1> <star2>

+ _
@ my name is <star>

+ tên [của] (@him) là *
@ my name is <star>

+ tên gọi của (@him) là *
@ my name is <star>

+ gọi (@him) là *
@ my name is <star>

+ hãy gọi (@him) là *
@ my name is <star>

+ tên gọi của (@him) là *
@ my name is <star>

+ không nói *
@ không

+ không *
@ không

+ không
- Ok. {sentence}<bot callhim>{/sentence} có thể nói sau được mà.{topic=random}
- Rất vui được hỗ trợ <bot callhim>.{topic=random}

< topic

// Substitutions (VI) : ưu tiên phạm vi & tần suất sử dụng
! sub +         = cộng
! sub -         = trừ
! sub *         = nhân
! sub /         = chia
! sub k			= không
! sub ko		= không
! sub k0		= không
! sub khong		= không
! sub khogn		= không
! sub khog		= không
! sub bi h		= bây giờ
! sub rui		= rồi
! sub rùi		= rồi
! sub bit rui	= biết rồi
! sub bit rùi	= biết rồi
! sub j			= gì
! sub dj		= đi
! sub xog		= xong
! sub uh		= ừ
! sub uhh		= ừ
! sub đc		= được
! sub ohh		= oh
! sub chát		= chat

// Substitutions (EN) : ưu tiên tần suất sử dụng
! sub &quot;    = "
! sub &apos;    = '
! sub &amp;     = &
! sub &lt;      = <
! sub &gt;      = >
! sub if		= nếu
! sub u			= bạn
! sub you		= bạn
! sub thanks	= cám ơn
! sub thank's	= cám ơn
! sub ok		= đồng ý
! sub okay		= đồng ý
! sub good		= tốt
! sub uh-huh	= đồng ý
! sub lol		= cười to
! sub i'm       = i am
! sub i'd       = i would
! sub i've      = i have
! sub i'll      = i will
! sub don't     = do not
! sub isn't     = is not
! sub you'd     = you would
! sub you're    = you are
! sub you've    = you have
! sub you'll    = you will
! sub he'd      = he would
! sub he's      = he is
! sub he'll     = he will
! sub she'd     = she would
! sub she's     = she is
! sub she'll    = she will
! sub they'd    = they would
! sub they're   = they are
! sub they've   = they have
! sub they'll   = they will
! sub we'd      = we would
! sub we're     = we are
! sub we've     = we have
! sub we'll     = we will
! sub whats     = what is
! sub what's    = what is
! sub what're   = what are
! sub what've   = what have
! sub what'll   = what will
! sub can't     = can not
! sub whos      = who is
! sub who's     = who is
! sub who'd     = who would
! sub who'll    = who will
! sub don't     = do not
! sub didn't    = did not
! sub it's      = it is
! sub could've  = could have
! sub should've = should have
! sub would've  = would have
! sub when's    = when is
! sub when're   = when are
! sub when'd    = when did
! sub u         = you
! sub ur        = your
! sub r         = are
! sub im        = i am
! sub wat       = what
! sub wats      = what is
! sub ohh       = oh
! sub becuse    = because
! sub becasue   = because
! sub becuase   = because
! sub practise  = practice
! sub its a     = it is a
! sub fav       = favorite
! sub fave      = favorite
! sub yesi      = yes i
! sub yetit     = yet it
! sub iam       = i am
! sub welli     = well i
! sub wellit    = well it
! sub amfine    = am fine
! sub aman      = am an
! sub amon      = am on
! sub amnot     = am not
! sub realy     = really
! sub iamusing  = i am using
! sub amleaving = am leaving
! sub yuo       = you
! sub youre     = you are
! sub didnt     = did not
! sub ain't     = is not
! sub aint      = is not
! sub wanna     = want to
! sub brb       = be right back
! sub bbl       = be back later
! sub gtg       = got to go
! sub g2g       = got to go
! sub lyl       = love you lots
! sub gf        = girlfriend
! sub g/f       = girlfriend
! sub bf        = boyfriend
! sub b/f       = boyfriend
! sub b/f/f     = best friend forever

// Substitutions (SYM) : ưu tiên ký tự hay sử dụng
/*  KHÔNG SỬ DỤNG
! sub :-)       = cười mỉm
! sub :)        = cười mỉm
! sub :d        = cười toe toét
! sub :-d       = cười toe toét
! sub :-p       = lè lưỡi
! sub :p        = lè lưỡi
! sub ;-)       = nháy mắt
! sub ;)        = nháy mắt
! sub :-(       = buồn
! sub :(        = buồn
! sub :'(       = khóc
! sub :-[       = nhút nhát
! sub :-\       = không chắc chắn
! sub :-/       = không chắc chắn
! sub :-s       = nghi ngờ
! sub :s		= nghi ngờ
! sub 8-)       = tuyệt
! sub 8)        = tuyệt
! sub :-*       = hôn má
! sub :-!       = ăn gạch
! sub o:-)      = đáng yêu
! sub >:o       = tức giận
! sub :@        = tức giận
! sub 8o|       = tức giận
! sub :$        = đỏ mặt
! sub :-$       = đỏ mặt
! sub :-[       = đỏ mặt
! sub :[        = ăn gạch
! sub (a)       = đáng yêu
! sub (h)       = cool
! sub 8-|       = nerdy
! sub |-)       = tired
! sub +o(       = ốm
! sub *-)       = lạ
! sub ^o)       = nheo mày
! sub (6)       = tức giận
! sub (l)       = yêu
! sub (u)       = vỡ tim
! sub (k)       = hôn má
! sub (f)       = tặng hoa
! sub (w)       = hoa héo
*/

// Person substitutions (VI)
! person anh	= em
! person em		= anh
! person chị	= em
! person tao	= mày
! person mày	= tao
! person cậu	= tớ

//! person của anh  = của em
//! person của em	= của anh
//! person của chị  = của em
//! person của mày  = của tao
//! person của tao  = của mày

// Person substitutions (EN)
! person i am    = you are
! person you are = I am
! person i'm     = you're
! person you're  = I'm
! person my      = your
! person your    = my
! person you     = I
! person i       = you

// Set arrays
! array me		= cô chú bạn anh chị em mày
! array him		= tao tôi anh chị em cháu
! array gender	= nam nữ
! array faction = con trai|con gái|đàn ông|đàn bà
! array supprise= ô|ồ|hay quá|tuyệt quá|tuyệt vời
! array malenoun   = con trai|đàn ông|quí ông|một quí ông|male guy boy dude boi man men gentleman gentlemen
! array femalenoun = female girl chick woman women lady babe
// Variables
! array malenoun   = nam|công tử|trai|con trai|đàn ông|quí ông|ông già|giống đực
! array femalenoun = nữ|công nương|gái|cô gái|con gái|đàn bà|phụ nữ|bà già|giống cái|female|girl|chick|woman|women|lady|babe
! array mennoun    = males guys boys dudes bois men gentlemen
! array womennoun  = females girls chicks women ladies babes
! array lol        = lol lmao rofl rotfl haha hahaha hihi
! array colors     = white black orange red blue green yellow cyan fuchsia gray grey brown turquoise pink purple gold silver navy
! array height     = tall long wide thick
! array measure    = inch in centimeter cm millimeter mm meter m inches centimeters millimeters meters
! array yes        = yes yeah yep yup ya yea
! array no         = no nah nope nay
