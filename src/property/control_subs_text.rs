#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoButtonText: Button {
		fn_name: do_text,
		impl_val: GodotString,
		property: "text",
		tween: TweenProperty_String,
	}
}

do_full_trait! {
	pub trait DoLinkButtonText: LinkButton {
		fn_name: do_text,
		impl_val: GodotString,
		property: "text",
		tween: TweenProperty_String,
	}
}

do_full_trait! {
	pub trait DoLabelText: Label {
		fn_name: do_text,
		impl_val: GodotString,
		property: "text",
		tween: TweenProperty_String,
	}
}

do_full_trait! {
	pub trait DoLabelVisibleCharacters: Label {
		fn_name: do_visible_characters,
		val: i64,
		property: "visible_characters",
		tween: TweenProperty_i64,
	}
}

do_full_trait! {
	pub trait DoLabelPercentVisible: Label {
		fn_name: do_percent_visible,
		val: f64,
		property: "percent_visible",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoLineEditText: LineEdit {
		fn_name: do_text,
		impl_val: GodotString,
		property: "text",
		tween: TweenProperty_String,
	}
}

do_full_trait! {
	pub trait DoLineEditPlaceholderText: LineEdit {
		fn_name: do_placeholder_text,
		impl_val: GodotString,
		property: "placeholder_text",
		tween: TweenProperty_String,
	}
}

do_full_trait! {
	pub trait DoRichTextLabelText: RichTextLabel {
		fn_name: do_text,
		impl_val: GodotString,
		property: "text",
		tween: TweenProperty_String,
	}
}

do_full_trait! {
	pub trait DoRichTextLabelBBCodeText: RichTextLabel {
		fn_name: do_bbcode_text,
		impl_val: GodotString,
		property: "bbcode_text",
		tween: TweenProperty_String,
	}
}

do_full_trait! {
	pub trait DoRichTextLabelVisibleCharacters: RichTextLabel {
		fn_name: do_visible_characters,
		val: i64,
		property: "visible_characters",
		tween: TweenProperty_i64,
	}
}

do_full_trait! {
	pub trait DoRichTextLabelPercentVisible: RichTextLabel {
		fn_name: do_percent_visible,
		val: f64,
		property: "percent_visible",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoTextEditText: TextEdit {
		fn_name: do_text,
		impl_val: GodotString,
		property: "text",
		tween: TweenProperty_String,
	}
}