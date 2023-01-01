use perseus::{web_log, Html, RenderFnResultWithCause, SsrNode, Template};
use serde::Deserialize;
use serde::Serialize;
use sycamore::prelude::{component, view, Keyed, KeyedProps, ReadSignal, Signal, View};

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResumeItemCTAState {
    pub text: String,
    pub href: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum ResumeItemVariant {
    ImageLeft,
    ImageRight,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResumeItemState {
    pub title: String,
    pub description: Option<String>,
    pub cta: Option<ResumeItemCTAState>,
    pub secondary_cta: Option<ResumeItemCTAState>,
    pub image: String,
    pub variant: ResumeItemVariant,
}

#[component(ResumeItem<G>)]
fn resume_item(state: ResumeItemState) -> View<G> {
    web_log!("component {}, variant {:?}", state.title, state.variant);
    let description = state.description.unwrap_or_default();
    let cta_view = match state.cta {
        Some(cta) => view! {
            a(class="resume-item--cta rounded-md p-3 border-solid border-2 border-sky-600 text-sky-700 inline-block", href=cta.href) { (cta.text)}
        },
        None => view! {},
    };
    let secondary_cta_view = match state.secondary_cta {
        Some(cta) => view! {
            a(class="resume-item--cta rounded-md ml-5 p-3 border-solid border-2 border-amber-600 text-amber-700 inline-block", href=cta.href) { (cta.text)}
        },
        None => view! {},
    };

    match state.variant {
        ResumeItemVariant::ImageLeft => view! {
            div(class="resume-item flex max-w-5xl mx-auto my-10 lg:my-16 items-center flex-col-reverse lg:flex-row") {
                div(class="resume-item__image-container basis-6/6 md:basis-3/6 ") {
                    img(class="resume-item--image ml-auto ml-5", src=state.image)
                }
                div(class="resume-item__content basis-6/6 md:basis-3/6 ml-5") {
                    h2(class="resume-item--title text-4xl mb-4") { (state.title) }
                    p(class="resume-item--description my-5") {(description)}
                    (cta_view)
                    (secondary_cta_view)
                }
            }
        },
        ResumeItemVariant::ImageRight => view! {
            div(class="resume-item flex max-w-5xl mx-auto my-10 lg:my-16 items-center flex-col lg:flex-row") {
                div(class="resume-item__content basis-6/6 md:basis-3/6 ml-5") {
                    h2(class="resume-item--title text-4xl mb-4") { (state.title) }
                    p(class="resume-item--description my-5") {(description)}
                    (cta_view)
                    (secondary_cta_view)
                }
                div(class="resume-item__image-container basis-6/6 md:basis-3/6 ") {
                    img(class="resume-item--image ml-auto ml-5", src=state.image)
                }
            }
        },
    }
}

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub name: String,
    pub logo: String,
    pub subtext: String,
    pub items: Vec<ResumeItemState>,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
    view! {
        div(class="container mx-auto") {
            div(class="mb-7 lg:mb-10") {
                img(src=state.logo.get(), alt="Logo of vpurush", class="w-2/3 lg:w-1/3 p-4 mx-auto")
            }
            p(class="my-7 lg:my-10 px-5 text-center text-1xl max-w-5xl mx-auto") { (state.subtext.get()) }
            Keyed( KeyedProps {
                    iterable: state.items.handle(),
                    template: |item| view! {
                        ResumeItem(item)
                    },
                    key: |item| ((&item.title).to_string()),
                }
            )
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Resume | Vijay Purush" }
        link (href="/.perseus/static/main.css", rel="stylesheet")
        script(src="https://cdn.tailwindcss.com")
        meta(itemprop="name", content="Vijay Purush's profile")
        meta(itemprop="description", content="Vijay is a continuous learner with a facination for underlying technology more than its applications.")
        meta(itemprop="image", content="https://profile.vpurush.com/.perseus/static/vpurush.png")
        link (rel="canonical", href="https://profile.vpurush.com")
        meta(property="og:title", content="Vijay Purush's profile")
        meta(property="og:type", content="profile")
        meta(property="og:url", content="https://profile.vpurush.com")
        meta(property="og:image", content="https://profile.vpurush.com/.perseus/static/vpurush.png")
        meta(property="og:description", content="Vijay is a continuous learner with a facination for underlying technology more than its applications")
        script(type="application/ld+json"){ "{
            \"@context\": \"http://schema.org/\",
            \"@type\": \"Person\",
            \"name\": \"Vijay Purush\",
            \"jobTitle\": \"Software developer\",
            \"url\": \"https://profile.vpurush.com\"
          }"}
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        name: "Vijay Purush".to_string(),
        logo: "/.perseus/static/vpurush.png".to_string(),
        subtext: "I hope this page categorically expounds my take on various aspects of development and thereby presents my experience. This is by no means an exhaustive list of technologies I have worked on in the past, just the recent ones.".to_string(),
        items: vec![ResumeItemState {
            title: "Programming languages".to_string(),
            image: "/.perseus/static/pl.png".to_string(),
            description: Some(
                "The dynamically-typed nature of Javascript impressed me early on when I used to be a C# developer. Just when I was struggling to cope with the bugs caused by the lack of static types, Typescript turned out to be a breath of fresh air. \n But it was not until I started learning Rust that I realized the true power of type safety and found my favourite programming language."
                    .to_string(),
            ),
            cta: None,
            secondary_cta: None,
            variant: ResumeItemVariant::ImageRight
        },ResumeItemState {
            title: "UI Frameworks and libraries".to_string(),
            image: "/.perseus/static/ui-framework.png".to_string(),
            description: Some(
                "I have found React to be a pretty good library for creating complex web applications with redux-saga. Needless to say, that React when combined with frameworks such as GatsbyJS or NextJS can be more than suitable for creating static websities. I can't say that I am a fan of VueJS & Nuxt but I am indeed facinated by Perseus & Sycamore at their attempt at static websites using Rust."
                    .to_string(),
            ),
            cta: None,
            secondary_cta: None,
            variant: ResumeItemVariant::ImageLeft
        },ResumeItemState {
            title: "Backend Frameworks".to_string(),
            image: "/.perseus/static/be-framework.png".to_string(),
            description: Some(
                "Having used Node.JS in serverless, container and dedicated server environments, I am both impressed with and have realized the limitations of JIT (just in time) compilation which is why I am now keen to explore other webservers, especially those used in conjunction with Rust. Exposing a popular CMS sdk as a GraphQL endpoint has taught me enough to apppreciate the purpose of GraphQL in large-scale applications."
                    .to_string(),
            ),
            cta: None,
            secondary_cta: None,
            variant: ResumeItemVariant::ImageRight
        },ResumeItemState {
            title: "Development process".to_string(),
            image: "/.perseus/static/agile.png".to_string(),
            description: Some(
                "Project management is not my forte but I firmly believe that every team member should have an understanding of the process that allows them to question the status quo when something is not right. I used to be a fan of the scrum methodology but having also worked in a hybrid (scrum and kanban) model for sometime now, I can clearly see the benefits of kanban."
                    .to_string(),
            ),
            cta: None,
            secondary_cta: None,
            variant: ResumeItemVariant::ImageLeft
        },ResumeItemState {
            title: "Certifications".to_string(),
            image: "/.perseus/static/certi.png".to_string(),
            description: Some(
                "I am an AWS certified developer associate and a Contentful certified professional, besides the various expired certifications acquired during my career."
                    .to_string(),
            ),
            cta: Some(ResumeItemCTAState {
                href: "https://www.linkedin.com/in/vpurush/details/certifications/".to_string(),
                text: "Certifications".to_string()
            }),
            secondary_cta: None,
            variant: ResumeItemVariant::ImageRight
        },ResumeItemState {
            title: "Contact".to_string(),
            image: "/.perseus/static/contact.png".to_string(),
            description: Some(
                "Feel free to contact me if you have something worthwhile to say. I hope whatever you have got to say is beneficial to both of us."
                    .to_string(),
            ),
            cta: Some(ResumeItemCTAState {
                href: "https://www.linkedin.com/in/vpurush/".to_string(),
                text: "Linkedin".to_string()
            }),
            secondary_cta: Some(ResumeItemCTAState {
                href: "https://github.com/vpurush".to_string(),
                text: "Github".to_string()
            }),
            variant: ResumeItemVariant::ImageLeft
        },ResumeItemState {
            title: "This site".to_string(),
            image: "/.perseus/static/site.png".to_string(),
            description: Some(
                "The images on this web page were created using Concepts App on iOS. HTML and WASM were generated using Sycamore & Perseus. Custom CSS were hand written along with Tailwind."
                    .to_string(),
            ),
            cta: Some(ResumeItemCTAState {
                href: "https://github.com/vpurush/web-resume-rust".to_string(),
                text: "Site's github repository".to_string()
            }),
            secondary_cta: None,
            variant: ResumeItemVariant::ImageRight
        }],
    })
}
