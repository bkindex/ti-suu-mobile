+ *
* <get email> eq undefined => {sentence}<bot callme>{/sentence} sẽ nhờ các chuyên gia giải đáp cho <bot callhim> sau. Email của <bot callhim> là gì?{topic=email_support}
- {sentence}<bot callme>{/sentence} sẽ nhờ các chuyên gia giải đáp cho <bot callhim> sau

> topic email_support inherits random

+ #
- Email của <bot callhim> không phải là chữ số đấy chứ?

+ *
- {@<star>}{topic=random}

+ my email is *
- <set email=<star>>Địa chỉ của <bot callhim> là <get email>. Cám ơn <bot callhim> vì sự tin tưởng{topic=random}

+ * atsign * dot *
- <set email=<star1>@<star2>.<star3>>Địa chỉ của <bot callhim> là <get email>. Cám ơn <bot callhim> vì sự tin tưởng{topic=random}

+ * * atsign * dot *
@ <star2> atsign <star3> dot <star4>

+ * * * atsign * dot *
@ <star3> atsign <star4> dot <star5>

+ _
@ my email is <star>

+ (mail|email) của (@him) là *
@ my email is <star>

+ * là địa chỉ *
@ my email is <star>

+ * là email *
@ my email is <star>

+ địa chỉ của (@him) là *
@ my email is <star>

+ địa chỉ email của (@him) là *
@ my email is <star>

+ * là *
@ my email is <star2>

+ không nói *
% * email của * là gì
@ không

+ không
@ không nói

+ không *
@ không nói

+ không nói
- Oke.{topic=random}
- :){topic=random}

+ spam1
- Các thông tin cá nhân người dùng được lưu trữ với mục đích liên lạc và không cung cấp cho bên thứ 3.
^ \s{sentence}<bot callhim>{/sentence} có thể đọc các điều lệ để tham khảo

+ spam
- Tyệt đối không! {@spam1}

+ * spam *
@ spam1

+ * thông tin *
@ spam1

+ * an toàn *
@ spam1

+ * bảo mật *
@ spam1

< topic

> topic new_question_mail_support
+ (co|có|vâng)
- {sentence}<bot callme>{/sentence} cố gắng trả lời <bot callhim> sớm nhất có thể
> topic