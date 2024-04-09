use stylist::{css, Style};

/// Tailwind `animate-none`; css `animation: none;`
pub fn animate_none() -> Style {
    Style::new(css!("animation: none;")).unwrap()
}

/// Tailwind `animate-spin`; css `animation: spin 1s linear infinite; @keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }`
pub fn animate_spin() -> Style {
    Style::new(css!("animation: spin 1s linear infinite; @keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }")).unwrap()
}

/// Tailwind `animate-ping`; css `animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite; @keyframes ping { 75%, 100% { transform: scale(2); opacity: 0; } }`
pub fn animate_ping() -> Style {
    Style::new(css!("animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite; @keyframes ping { 75%, 100% { transform: scale(2); opacity: 0; } }")).unwrap()
}

/// Tailwind `animate-pulse`; css `animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: .5; } }`
pub fn animate_pulse() -> Style {
    Style::new(css!("animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: .5; } }")).unwrap()
}

/// Tailwind `animate-bounce`; css `animation: bounce 1s infinite; @keyframes bounce { 0%, 100% { transform: translateY(-25%); animation-timing-function: cubic-bezier(0.8, 0, 1, 1); } 50% { transform: translateY(0); animation-timing-function: cubic-bezier(0, 0, 0.2, 1); } }`
pub fn animate_bounce() -> Style {
    Style::new(css!("animation: bounce 1s infinite; @keyframes bounce { 0%, 100% { transform: translateY(-25%); animation-timing-function: cubic-bezier(0.8, 0, 1, 1); } 50% { transform: translateY(0); animation-timing-function: cubic-bezier(0, 0, 0.2, 1); } }")).unwrap()
}

