//! The OS secure store (Keychain, Secret Service, Credential Manager) for app secrets.
//! Values never enter prompts, logs, the database or the webview.

#[cfg(test)]
fn memory() -> &'static std::sync::Mutex<std::collections::HashMap<String, String>> {
    static STORE: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, String>>> = std::sync::OnceLock::new();
    STORE.get_or_init(Default::default)
}

pub(crate) fn set(service: &str, account: &str, value: &str) -> crate::Result<()> {
    #[cfg(feature = "beta-isolation")]
    {
        let _ = (service, account, value);
        return Err("Credential storage is disabled in the isolated beta build.".into());
    }
    #[cfg(not(feature = "beta-isolation"))]
    {
    let value = value.trim();
    if value.is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        return Err("That secret looks empty or malformed.".into());
    }
    #[cfg(test)]
    {
        memory().lock().unwrap().insert(format!("{service}/{account}"), value.to_owned());
        Ok(())
    }
    #[cfg(all(not(test), target_os = "macos"))]
    {
        security_framework::passwords::set_generic_password(service, account, value.as_bytes()).map_err(|_| "Could not save the secret in Keychain.".into())
    }
    #[cfg(all(not(test), any(target_os = "linux", target_os = "windows")))]
    {
        keyring::Entry::new(service, account).and_then(|entry| entry.set_password(value)).map_err(|_| "Could not save the secret in secure storage.".into())
    }
    #[cfg(all(not(test), not(any(target_os = "macos", target_os = "linux", target_os = "windows"))))]
    {
        let _ = (service, account, value);
        Err("Secure storage is not available on this platform.".into())
    }
    }
}

pub(crate) fn get(service: &str, account: &str) -> Option<String> {
    #[cfg(feature = "beta-isolation")]
    {
        let _ = (service, account);
        None
    }
    #[cfg(not(feature = "beta-isolation"))]
    {
    #[cfg(test)]
    {
        memory().lock().unwrap().get(&format!("{service}/{account}")).cloned()
    }
    #[cfg(all(not(test), target_os = "macos"))]
    {
        match security_framework::passwords::get_generic_password(service, account) {
            Ok(bytes) => String::from_utf8(bytes).ok(),
            // -25300 is "not found"; anything else (a denied or dismissed access prompt) is worth logging.
            Err(error) if error.code() == -25300 => None,
            Err(error) => {
                eprintln!("Keychain read for {service} failed ({}): {error}", error.code());
                None
            }
        }
    }
    #[cfg(all(not(test), any(target_os = "linux", target_os = "windows")))]
    {
        keyring::Entry::new(service, account).and_then(|entry| entry.get_password()).ok()
    }
    #[cfg(all(not(test), not(any(target_os = "macos", target_os = "linux", target_os = "windows"))))]
    {
        let _ = (service, account);
        None
    }
    }
}

pub(crate) fn remove(service: &str, account: &str) {
    #[cfg(feature = "beta-isolation")]
    {
        let _ = (service, account);
    }
    #[cfg(not(feature = "beta-isolation"))]
    {
    #[cfg(test)]
    {
        memory().lock().unwrap().remove(&format!("{service}/{account}"));
    }
    #[cfg(all(not(test), target_os = "macos"))]
    {
        let _ = security_framework::passwords::delete_generic_password(service, account);
    }
    #[cfg(all(not(test), any(target_os = "linux", target_os = "windows")))]
    {
        if let Ok(entry) = keyring::Entry::new(service, account) {
            let _ = entry.delete_credential();
        }
    }
    #[cfg(all(not(test), not(any(target_os = "macos", target_os = "linux", target_os = "windows"))))]
    {
        let _ = (service, account);
    }
    }
}

/// Credential-management revocation MUST surface OS deletion failures.
/// The older fire-and-forget remove() remains for noncritical legacy use.
pub(crate) fn remove_checked(service: &str, account: &str) -> crate::Result<()> {
    #[cfg(feature = "beta-isolation")]
    {
        let _ = (service, account);
        Err("Credential storage is disabled in the isolated beta build.".into())
    }
    #[cfg(not(feature = "beta-isolation"))]
    {
    #[cfg(test)]
    {
        memory().lock().map_err(|_| "Secure-store test double unavailable.")?
            .remove(&format!("{service}/{account}"));
        Ok(())
    }
    #[cfg(all(not(test), target_os = "macos"))]
    {
        security_framework::passwords::delete_generic_password(service, account)
            .map_err(|_| "Could not revoke credential in macOS Keychain.".into())
    }
    #[cfg(all(not(test), any(target_os = "linux", target_os = "windows")))]
    {
        keyring::Entry::new(service, account)
            .and_then(|entry| entry.delete_credential())
            .map_err(|_| "Could not revoke credential in OS secure storage.".into())
    }
    #[cfg(all(not(test), not(any(target_os = "macos", target_os = "linux", target_os = "windows"))))]
    {
        let _ = (service, account);
        Err("OS secure storage is unavailable.".into())
    }
    }
}

#[cfg(all(test, feature = "beta-isolation"))]
mod beta_tests {
    use super::*;

    #[test]
    fn beta_store_fails_closed_without_mutating_any_store() {
        assert!(set("legacy.service", "account", "secret").is_err());
        assert_eq!(get("legacy.service", "account"), None);
        remove("legacy.service", "account");
        assert!(remove_checked("legacy.service", "account").is_err());
    }
}
