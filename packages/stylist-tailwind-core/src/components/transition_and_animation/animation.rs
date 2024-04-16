use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "animate-none" => "animation: none;",
    "animate-spin" => "animation: spin 1s linear infinite; @keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }",
    "animate-ping" => "animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite; @keyframes ping { 75%, 100% { transform: scale(2); opacity: 0; } }",
    "animate-pulse" => "animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: .5; } }",
    "animate-bounce" => "animation: bounce 1s infinite; @keyframes bounce { 0%, 100% { transform: translateY(-25%); animation-timing-function: cubic-bezier(0.8, 0, 1, 1); } 50% { transform: translateY(0); animation-timing-function: cubic-bezier(0, 0, 0.2, 1); } }",
};