use crate::{Context, Data, Error};

type FluentBundle = fluent::bundle::FluentBundle<
    fluent::FluentResource,
    intl_memoizer::concurrent::IntlLangMemoizer,
>;

pub struct Translations {
    main: FluentBundle,
    other: std::collections::HashMap<String, FluentBundle>,
}

macro_rules! translate {
    ( $ctx:ident, $id:expr $(, $argname:ident: $argvalue:expr )* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut args = fluent::FluentArgs::new();
        $( args.set(stringify!($argname), $argvalue); )*

        $crate::components::translation::get($ctx, $id, None, Some(&args))
    }};
}

use tracing::info;
pub(crate) use translate;

pub fn format(
    bundle: &FluentBundle,
    id: &str,
    attr: Option<&str>,
    args: Option<&fluent::FluentArgs<'_>>,
) -> Option<String> {
    let message = bundle.get_message(id)?;
    let pattern = match attr {
        Some(attribute) => message.get_attribute(attribute)?.value(),
        None => message.value()?,
    };
    let formatted = bundle.format_pattern(pattern, args, &mut vec![]);
    Some(formatted.into_owned())
}

pub fn get(
    ctx: Context,
    id: &str,
    attr: Option<&str>,
    args: Option<&fluent::FluentArgs<'_>>,
) -> String {
    let translations = &ctx.data().translations;
    ctx.locale()
        .and_then(|locale| format(translations.other.get(locale)?, id, attr, args))
        .or_else(|| format(&translations.main, id, attr, args))
        .unwrap_or_else(|| {
            tracing::warn!("Unknown fluent message identifier `{}`", id);
            id.to_string()
        })
}

pub fn read_ftl() -> Result<Translations, Error> {
    fn read_single_ftl(path: &std::path::Path) -> Result<(String, FluentBundle), Error> {
        let locale = path.file_stem().ok_or("Invalid .ftl filename")?;
        let locale = locale.to_str().ok_or("Invalid filename UTF-8")?;

        let file_contents = std::fs::read_to_string(path)?;
        let resource = fluent::FluentResource::try_new(file_contents)
            .map_err(|(_, e)| format!("Failed to parse {:?}: {:?}", path, e))?;

        let mut bundle = FluentBundle::new_concurrent(vec![locale
            .parse()
            .map_err(|e| format!("Invalid locale `{}`: {}", locale, e))?]);

        bundle
            .add_resource(resource)
            .map_err(|e| format!("Failed to add resource to bundle: {:?}", e))?;

        Ok((locale.to_string(), bundle))
    }

    Ok(Translations {
        main: read_single_ftl("translations/en-US.ftl".as_ref())?.1,
        other: std::fs::read_dir("translations")?
            .map(|file| read_single_ftl(&file?.path()))
            .collect::<Result<_, _>>()?,
    })
}

pub fn apply_translations(
    translations: &Translations,
    commands: &mut [poise::Command<Data, Error>],
) {
    for command in &mut *commands {
        for (locale, bundle) in &translations.other {
            let localized_command_name = match format(bundle, &command.name, None, None) {
                Some(x) => x,
                None => continue,
            };

            command
                .name_localizations
                .insert(locale.clone(), localized_command_name);

            command.description_localizations.insert(
                locale.clone(),
                format(bundle, &command.name, Some("description"), None).unwrap(),
            );

            for parameter in &mut command.parameters {
                parameter.name_localizations.insert(
                    locale.clone(),
                    format(bundle, &command.name, Some(&parameter.name), None).unwrap(),
                );

                parameter.description_localizations.insert(
                    locale.clone(),
                    format(
                        bundle,
                        &command.name,
                        Some(&format!("{}-description", parameter.name)),
                        None,
                    )
                    .unwrap(),
                );

                for choice in &mut parameter.choices {
                    choice.localizations.insert(
                        locale.clone(),
                        format(bundle, &choice.name, None, None).unwrap(),
                    );
                }
            }

            for subcommand in &mut command.subcommands {
                let subcommand_key = format!("{}-{}", &command.name, &subcommand.name);

                let localized_subcommand_name = match format(bundle, &subcommand_key, None, None) {
                    Some(x) => x,
                    None => continue,
                };

                subcommand
                    .name_localizations
                    .insert(locale.clone(), localized_subcommand_name);

                subcommand.description_localizations.insert(
                    locale.clone(),
                    format(bundle, &subcommand_key, Some("description"), None).unwrap(),
                );

                for parameter in &mut subcommand.parameters {
                    parameter.name_localizations.insert(
                        locale.clone(),
                        format(bundle, &subcommand_key, Some(&parameter.name), None).unwrap(),
                    );

                    parameter.description_localizations.insert(
                        locale.clone(),
                        format(
                            bundle,
                            &subcommand_key,
                            Some(&format!("{}-description", parameter.name)),
                            None,
                        )
                        .unwrap(),
                    );

                    for choice in &mut parameter.choices {
                        choice.localizations.insert(
                            locale.clone(),
                            format(bundle, &choice.name, None, None).unwrap(),
                        );
                    }
                }
            }
        }

        let bundle = &translations.main;
        match format(bundle, &command.name, None, None) {
            Some(x) => command.name = x,
            None => continue,
        }

        command.description =
            Some(format(bundle, &command.name, Some("description"), None).unwrap());

        for parameter in &mut command.parameters {
            parameter.name = format(bundle, &command.name, Some(&parameter.name), None).unwrap();
            parameter.description = Some(
                format(
                    bundle,
                    &command.name,
                    Some(&format!("{}-description", parameter.name)),
                    None,
                )
                .unwrap(),
            );

            for choice in &mut parameter.choices {
                choice.name = format(bundle, &choice.name, None, None).unwrap();
            }
        }

        for subcommand in &mut command.subcommands {
            let subcommand_key = format!("{}-{}", &command.name, &subcommand.name);

            subcommand.name = format(bundle, &subcommand_key, None, None).unwrap();

            subcommand.description =
                Some(format(bundle, &subcommand_key, Some("description"), None).unwrap());

            for parameter in &mut subcommand.parameters {
                parameter.name =
                    format(bundle, &subcommand_key, Some(&parameter.name), None).unwrap();

                parameter.description = Some(
                    format(
                        bundle,
                        &subcommand_key,
                        Some(&format!("{}-description", parameter.name)),
                        None,
                    )
                    .unwrap(),
                );

                for choice in &mut parameter.choices {
                    choice.name = format(bundle, &choice.name, None, None).unwrap();
                }
            }
        }
    }
}
