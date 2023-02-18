/// Implement the methods of the `StreamTrait` and `ObjectTrait` for any struct
/// containing a `base: Object` field.
#[macro_export]
macro_rules! impl_Object_for {
    ($t:ident) => {
        #[allow(non_snake_case)]
        impl $crate::traits::StreamTrait for $t {
            fn atContext(&mut self, value: property::AtContext) -> Self {
                self.base.atContext = Some(value);
                self.to_owned()
            }
            fn id(&mut self, value: String) -> Self {
                self.base.id = Some(value);
                self.to_owned()
            }
            fn r#type(&mut self, value: property::Type) -> Self {
                self.base.r#type = Some(value);
                self.to_owned()
            }
        }

        #[allow(non_snake_case)]
        impl $crate::traits::ObjectTrait for $t {
            fn attachment(&mut self, value: property::Attachment) -> Self {
                self.base.attachment = Some(value);
                self.to_owned()
            }
            fn attributedTo(&mut self, value: property::AttributedTo) -> Self {
                self.base.attributedTo = Some(value);
                self.to_owned()
            }
            fn audience(&mut self, value: property::Audience) -> Self {
                self.base.audience = Some(value);
                self.to_owned()
            }
            fn bcc(&mut self, value: property::Bcc) -> Self {
                self.base.bcc = Some(value);
                self.to_owned()
            }
            fn bto(&mut self, value: property::Bto) -> Self {
                self.base.bto = Some(value);
                self.to_owned()
            }
            fn cc(&mut self, value: property::Cc) -> Self {
                self.base.cc = Some(value);
                self.to_owned()
            }
            fn content(&mut self, value: String) -> Self {
                self.base.content = Some(value);
                self.to_owned()
            }
            fn contentMap(&mut self, value: std::collections::HashMap<String, String>) -> Self {
                self.base.contentMap = Some(value);
                self.to_owned()
            }
            fn context(&mut self, value: property::Context) -> Self {
                self.base.context = Some(value);
                self.to_owned()
            }
            fn duration(&mut self, value: String) -> Self {
                self.base.duration = Some(value);
                self.to_owned()
            }
            fn endTime(&mut self, value: String) -> Self {
                self.base.endTime = Some(value);
                self.to_owned()
            }
            fn generator(&mut self, value: property::Generator) -> Self {
                self.base.generator = Some(value);
                self.to_owned()
            }
            fn icon(&mut self, value: property::Icon) -> Self {
                self.base.icon = Some(value);
                self.to_owned()
            }
            fn image(&mut self, value: property::Image) -> Self {
                self.base.image = Some(value);
                self.to_owned()
            }
            fn inReplyTo(&mut self, value: property::InReplyTo) -> Self {
                self.base.inReplyTo = Some(value);
                self.to_owned()
            }
            fn location(&mut self, value: property::Location) -> Self {
                self.base.location = Some(value);
                self.to_owned()
            }
            fn mediaType(&mut self, value: String) -> Self {
                self.base.mediaType = Some(value);
                self.to_owned()
            }
            fn name(&mut self, value: String) -> Self {
                self.base.name = Some(value);
                self.to_owned()
            }
            fn nameMap(&mut self, value: std::collections::HashMap<String, String>) -> Self {
                self.base.nameMap = Some(value);
                self.to_owned()
            }
            fn preview(&mut self, value: property::Preview) -> Self {
                self.base.preview = Some(value);
                self.to_owned()
            }
            fn published(&mut self, value: String) -> Self {
                self.base.published = Some(value);
                self.to_owned()
            }
            fn replies(&mut self, value: property::Replies) -> Self {
                self.base.replies = Some(value);
                self.to_owned()
            }
            fn startTime(&mut self, value: String) -> Self {
                self.base.startTime = Some(value);
                self.to_owned()
            }
            fn summary(&mut self, value: String) -> Self {
                self.base.summary = Some(value);
                self.to_owned()
            }
            fn summaryMap(&mut self, value: std::collections::HashMap<String, String>) -> Self {
                self.base.summaryMap = Some(value);
                self.to_owned()
            }
            fn tag(&mut self, value: property::Tag) -> Self {
                self.base.tag = Some(value);
                self.to_owned()
            }
            fn to(&mut self, value: property::To) -> Self {
                self.base.to = Some(value);
                self.to_owned()
            }
            fn updated(&mut self, value: String) -> Self {
                self.base.updated = Some(value);
                self.to_owned()
            }
            fn url(&mut self, value: property::Url) -> Self {
                self.base.url = Some(value);
                self.to_owned()
            }
        }
    };
}
