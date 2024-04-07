use stylist::{css, Style};

/// Tailwind `from-inherit`; css `--tw-gradient-from: inherit var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_inherit() -> Style {
    Style::new(css!("--tw-gradient-from: inherit var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-current`; css `--tw-gradient-from: currentColor var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_current() -> Style {
    Style::new(css!("--tw-gradient-from: currentColor var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-transparent`; css `--tw-gradient-from: transparent var(--tw-gradient-from-position); --tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_transparent() -> Style {
    Style::new(css!("--tw-gradient-from: transparent var(--tw-gradient-from-position); --tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-black`; css `--tw-gradient-from: #000 var(--tw-gradient-from-position); --tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_black() -> Style {
    Style::new(css!("--tw-gradient-from: #000 var(--tw-gradient-from-position); --tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-white`; css `--tw-gradient-from: #fff var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_white() -> Style {
    Style::new(css!("--tw-gradient-from: #fff var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-50`; css `--tw-gradient-from: #f8fafc var(--tw-gradient-from-position); --tw-gradient-to: rgb(248 250 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_50() -> Style {
    Style::new(css!("--tw-gradient-from: #f8fafc var(--tw-gradient-from-position); --tw-gradient-to: rgb(248 250 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-100`; css `--tw-gradient-from: #f1f5f9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(241 245 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_100() -> Style {
    Style::new(css!("--tw-gradient-from: #f1f5f9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(241 245 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-200`; css `--tw-gradient-from: #e2e8f0 var(--tw-gradient-from-position); --tw-gradient-to: rgb(226 232 240 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_200() -> Style {
    Style::new(css!("--tw-gradient-from: #e2e8f0 var(--tw-gradient-from-position); --tw-gradient-to: rgb(226 232 240 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-300`; css `--tw-gradient-from: #cbd5e1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(203 213 225 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_300() -> Style {
    Style::new(css!("--tw-gradient-from: #cbd5e1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(203 213 225 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-400`; css `--tw-gradient-from: #94a3b8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(148 163 184 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_400() -> Style {
    Style::new(css!("--tw-gradient-from: #94a3b8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(148 163 184 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-500`; css `--tw-gradient-from: #64748b var(--tw-gradient-from-position); --tw-gradient-to: rgb(100 116 139 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_500() -> Style {
    Style::new(css!("--tw-gradient-from: #64748b var(--tw-gradient-from-position); --tw-gradient-to: rgb(100 116 139 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-600`; css `--tw-gradient-from: #475569 var(--tw-gradient-from-position); --tw-gradient-to: rgb(71 85 105 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_600() -> Style {
    Style::new(css!("--tw-gradient-from: #475569 var(--tw-gradient-from-position); --tw-gradient-to: rgb(71 85 105 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-700`; css `--tw-gradient-from: #334155 var(--tw-gradient-from-position); --tw-gradient-to: rgb(51 65 85 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_700() -> Style {
    Style::new(css!("--tw-gradient-from: #334155 var(--tw-gradient-from-position); --tw-gradient-to: rgb(51 65 85 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-800`; css `--tw-gradient-from: #1e293b var(--tw-gradient-from-position); --tw-gradient-to: rgb(30 41 59 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_800() -> Style {
    Style::new(css!("--tw-gradient-from: #1e293b var(--tw-gradient-from-position); --tw-gradient-to: rgb(30 41 59 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-900`; css `--tw-gradient-from: #0f172a var(--tw-gradient-from-position); --tw-gradient-to: rgb(15 23 42 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_900() -> Style {
    Style::new(css!("--tw-gradient-from: #0f172a var(--tw-gradient-from-position); --tw-gradient-to: rgb(15 23 42 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-slate-950`; css `--tw-gradient-from: #020617 var(--tw-gradient-from-position); --tw-gradient-to: rgb(2 6 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_slate_950() -> Style {
    Style::new(css!("--tw-gradient-from: #020617 var(--tw-gradient-from-position); --tw-gradient-to: rgb(2 6 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-50`; css `--tw-gradient-from: #f9fafb var(--tw-gradient-from-position); --tw-gradient-to: rgb(249 250 251 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_50() -> Style {
    Style::new(css!("--tw-gradient-from: #f9fafb var(--tw-gradient-from-position); --tw-gradient-to: rgb(249 250 251 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-100`; css `--tw-gradient-from: #f3f4f6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(243 244 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_100() -> Style {
    Style::new(css!("--tw-gradient-from: #f3f4f6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(243 244 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-200`; css `--tw-gradient-from: #e5e7eb var(--tw-gradient-from-position); --tw-gradient-to: rgb(229 231 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_200() -> Style {
    Style::new(css!("--tw-gradient-from: #e5e7eb var(--tw-gradient-from-position); --tw-gradient-to: rgb(229 231 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-300`; css `--tw-gradient-from: #d1d5db var(--tw-gradient-from-position); --tw-gradient-to: rgb(209 213 219 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_300() -> Style {
    Style::new(css!("--tw-gradient-from: #d1d5db var(--tw-gradient-from-position); --tw-gradient-to: rgb(209 213 219 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-400`; css `--tw-gradient-from: #9ca3af var(--tw-gradient-from-position); --tw-gradient-to: rgb(156 163 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_400() -> Style {
    Style::new(css!("--tw-gradient-from: #9ca3af var(--tw-gradient-from-position); --tw-gradient-to: rgb(156 163 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-500`; css `--tw-gradient-from: #6b7280 var(--tw-gradient-from-position); --tw-gradient-to: rgb(107 114 128 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_500() -> Style {
    Style::new(css!("--tw-gradient-from: #6b7280 var(--tw-gradient-from-position); --tw-gradient-to: rgb(107 114 128 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-600`; css `--tw-gradient-from: #4b5563 var(--tw-gradient-from-position); --tw-gradient-to: rgb(75 85 99 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_600() -> Style {
    Style::new(css!("--tw-gradient-from: #4b5563 var(--tw-gradient-from-position); --tw-gradient-to: rgb(75 85 99 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-700`; css `--tw-gradient-from: #374151 var(--tw-gradient-from-position); --tw-gradient-to: rgb(55 65 81 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_700() -> Style {
    Style::new(css!("--tw-gradient-from: #374151 var(--tw-gradient-from-position); --tw-gradient-to: rgb(55 65 81 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-800`; css `--tw-gradient-from: #1f2937 var(--tw-gradient-from-position); --tw-gradient-to: rgb(31 41 55 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_800() -> Style {
    Style::new(css!("--tw-gradient-from: #1f2937 var(--tw-gradient-from-position); --tw-gradient-to: rgb(31 41 55 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-900`; css `--tw-gradient-from: #111827 var(--tw-gradient-from-position); --tw-gradient-to: rgb(17 24 39 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_900() -> Style {
    Style::new(css!("--tw-gradient-from: #111827 var(--tw-gradient-from-position); --tw-gradient-to: rgb(17 24 39 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-gray-950`; css `--tw-gradient-from: #030712 var(--tw-gradient-from-position); --tw-gradient-to: rgb(3 7 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_gray_950() -> Style {
    Style::new(css!("--tw-gradient-from: #030712 var(--tw-gradient-from-position); --tw-gradient-to: rgb(3 7 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-50`; css `--tw-gradient-from: #fafafa var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fafafa var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-100`; css `--tw-gradient-from: #f4f4f5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(244 244 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_100() -> Style {
    Style::new(css!("--tw-gradient-from: #f4f4f5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(244 244 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-200`; css `--tw-gradient-from: #e4e4e7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(228 228 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_200() -> Style {
    Style::new(css!("--tw-gradient-from: #e4e4e7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(228 228 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-300`; css `--tw-gradient-from: #d4d4d8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(212 212 216 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_300() -> Style {
    Style::new(css!("--tw-gradient-from: #d4d4d8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(212 212 216 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-400`; css `--tw-gradient-from: #a1a1aa var(--tw-gradient-from-position); --tw-gradient-to: rgb(161 161 170 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_400() -> Style {
    Style::new(css!("--tw-gradient-from: #a1a1aa var(--tw-gradient-from-position); --tw-gradient-to: rgb(161 161 170 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-500`; css `--tw-gradient-from: #71717a var(--tw-gradient-from-position); --tw-gradient-to: rgb(113 113 122 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_500() -> Style {
    Style::new(css!("--tw-gradient-from: #71717a var(--tw-gradient-from-position); --tw-gradient-to: rgb(113 113 122 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-600`; css `--tw-gradient-from: #52525b var(--tw-gradient-from-position); --tw-gradient-to: rgb(82 82 91 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_600() -> Style {
    Style::new(css!("--tw-gradient-from: #52525b var(--tw-gradient-from-position); --tw-gradient-to: rgb(82 82 91 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-700`; css `--tw-gradient-from: #3f3f46 var(--tw-gradient-from-position); --tw-gradient-to: rgb(63 63 70 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_700() -> Style {
    Style::new(css!("--tw-gradient-from: #3f3f46 var(--tw-gradient-from-position); --tw-gradient-to: rgb(63 63 70 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-800`; css `--tw-gradient-from: #27272a var(--tw-gradient-from-position); --tw-gradient-to: rgb(39 39 42 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_800() -> Style {
    Style::new(css!("--tw-gradient-from: #27272a var(--tw-gradient-from-position); --tw-gradient-to: rgb(39 39 42 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-900`; css `--tw-gradient-from: #18181b var(--tw-gradient-from-position); --tw-gradient-to: rgb(24 24 27 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_900() -> Style {
    Style::new(css!("--tw-gradient-from: #18181b var(--tw-gradient-from-position); --tw-gradient-to: rgb(24 24 27 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-zinc-950`; css `--tw-gradient-from: #09090b var(--tw-gradient-from-position); --tw-gradient-to: rgb(9 9 11 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_zinc_950() -> Style {
    Style::new(css!("--tw-gradient-from: #09090b var(--tw-gradient-from-position); --tw-gradient-to: rgb(9 9 11 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-50`; css `--tw-gradient-from: #fafafa var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fafafa var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-100`; css `--tw-gradient-from: #f5f5f5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 245 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_100() -> Style {
    Style::new(css!("--tw-gradient-from: #f5f5f5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 245 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-200`; css `--tw-gradient-from: #e5e5e5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(229 229 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_200() -> Style {
    Style::new(css!("--tw-gradient-from: #e5e5e5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(229 229 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-300`; css `--tw-gradient-from: #d4d4d4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(212 212 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_300() -> Style {
    Style::new(css!("--tw-gradient-from: #d4d4d4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(212 212 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-400`; css `--tw-gradient-from: #a3a3a3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(163 163 163 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_400() -> Style {
    Style::new(css!("--tw-gradient-from: #a3a3a3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(163 163 163 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-500`; css `--tw-gradient-from: #737373 var(--tw-gradient-from-position); --tw-gradient-to: rgb(115 115 115 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_500() -> Style {
    Style::new(css!("--tw-gradient-from: #737373 var(--tw-gradient-from-position); --tw-gradient-to: rgb(115 115 115 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-600`; css `--tw-gradient-from: #525252 var(--tw-gradient-from-position); --tw-gradient-to: rgb(82 82 82 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_600() -> Style {
    Style::new(css!("--tw-gradient-from: #525252 var(--tw-gradient-from-position); --tw-gradient-to: rgb(82 82 82 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-700`; css `--tw-gradient-from: #404040 var(--tw-gradient-from-position); --tw-gradient-to: rgb(64 64 64 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_700() -> Style {
    Style::new(css!("--tw-gradient-from: #404040 var(--tw-gradient-from-position); --tw-gradient-to: rgb(64 64 64 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-800`; css `--tw-gradient-from: #262626 var(--tw-gradient-from-position); --tw-gradient-to: rgb(38 38 38 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_800() -> Style {
    Style::new(css!("--tw-gradient-from: #262626 var(--tw-gradient-from-position); --tw-gradient-to: rgb(38 38 38 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-900`; css `--tw-gradient-from: #171717 var(--tw-gradient-from-position); --tw-gradient-to: rgb(23 23 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_900() -> Style {
    Style::new(css!("--tw-gradient-from: #171717 var(--tw-gradient-from-position); --tw-gradient-to: rgb(23 23 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-neutral-950`; css `--tw-gradient-from: #0a0a0a var(--tw-gradient-from-position); --tw-gradient-to: rgb(10 10 10 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_neutral_950() -> Style {
    Style::new(css!("--tw-gradient-from: #0a0a0a var(--tw-gradient-from-position); --tw-gradient-to: rgb(10 10 10 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-50`; css `--tw-gradient-from: #fafaf9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 250 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fafaf9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 250 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-100`; css `--tw-gradient-from: #f5f5f4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 245 244 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_100() -> Style {
    Style::new(css!("--tw-gradient-from: #f5f5f4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 245 244 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-200`; css `--tw-gradient-from: #e7e5e4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(231 229 228 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_200() -> Style {
    Style::new(css!("--tw-gradient-from: #e7e5e4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(231 229 228 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-300`; css `--tw-gradient-from: #d6d3d1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(214 211 209 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_300() -> Style {
    Style::new(css!("--tw-gradient-from: #d6d3d1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(214 211 209 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-400`; css `--tw-gradient-from: #a8a29e var(--tw-gradient-from-position); --tw-gradient-to: rgb(168 162 158 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_400() -> Style {
    Style::new(css!("--tw-gradient-from: #a8a29e var(--tw-gradient-from-position); --tw-gradient-to: rgb(168 162 158 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-500`; css `--tw-gradient-from: #78716c var(--tw-gradient-from-position); --tw-gradient-to: rgb(120 113 108 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_500() -> Style {
    Style::new(css!("--tw-gradient-from: #78716c var(--tw-gradient-from-position); --tw-gradient-to: rgb(120 113 108 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-600`; css `--tw-gradient-from: #57534e var(--tw-gradient-from-position); --tw-gradient-to: rgb(87 83 78 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_600() -> Style {
    Style::new(css!("--tw-gradient-from: #57534e var(--tw-gradient-from-position); --tw-gradient-to: rgb(87 83 78 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-700`; css `--tw-gradient-from: #44403c var(--tw-gradient-from-position); --tw-gradient-to: rgb(68 64 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_700() -> Style {
    Style::new(css!("--tw-gradient-from: #44403c var(--tw-gradient-from-position); --tw-gradient-to: rgb(68 64 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-800`; css `--tw-gradient-from: #292524 var(--tw-gradient-from-position); --tw-gradient-to: rgb(41 37 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_800() -> Style {
    Style::new(css!("--tw-gradient-from: #292524 var(--tw-gradient-from-position); --tw-gradient-to: rgb(41 37 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-900`; css `--tw-gradient-from: #1c1917 var(--tw-gradient-from-position); --tw-gradient-to: rgb(28 25 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_900() -> Style {
    Style::new(css!("--tw-gradient-from: #1c1917 var(--tw-gradient-from-position); --tw-gradient-to: rgb(28 25 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-stone-950`; css `--tw-gradient-from: #0c0a09 var(--tw-gradient-from-position); --tw-gradient-to: rgb(12 10 9 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_stone_950() -> Style {
    Style::new(css!("--tw-gradient-from: #0c0a09 var(--tw-gradient-from-position); --tw-gradient-to: rgb(12 10 9 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-50`; css `--tw-gradient-from: #fef2f2 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 242 242 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fef2f2 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 242 242 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-100`; css `--tw-gradient-from: #fee2e2 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 226 226 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_100() -> Style {
    Style::new(css!("--tw-gradient-from: #fee2e2 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 226 226 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-200`; css `--tw-gradient-from: #fecaca var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 202 202 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_200() -> Style {
    Style::new(css!("--tw-gradient-from: #fecaca var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 202 202 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-300`; css `--tw-gradient-from: #fca5a5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(252 165 165 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_300() -> Style {
    Style::new(css!("--tw-gradient-from: #fca5a5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(252 165 165 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-400`; css `--tw-gradient-from: #f87171 var(--tw-gradient-from-position); --tw-gradient-to: rgb(248 113 113 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_400() -> Style {
    Style::new(css!("--tw-gradient-from: #f87171 var(--tw-gradient-from-position); --tw-gradient-to: rgb(248 113 113 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-500`; css `--tw-gradient-from: #ef4444 var(--tw-gradient-from-position); --tw-gradient-to: rgb(239 68 68 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_500() -> Style {
    Style::new(css!("--tw-gradient-from: #ef4444 var(--tw-gradient-from-position); --tw-gradient-to: rgb(239 68 68 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-600`; css `--tw-gradient-from: #dc2626 var(--tw-gradient-from-position); --tw-gradient-to: rgb(220 38 38 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_600() -> Style {
    Style::new(css!("--tw-gradient-from: #dc2626 var(--tw-gradient-from-position); --tw-gradient-to: rgb(220 38 38 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-700`; css `--tw-gradient-from: #b91c1c var(--tw-gradient-from-position); --tw-gradient-to: rgb(185 28 28 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_700() -> Style {
    Style::new(css!("--tw-gradient-from: #b91c1c var(--tw-gradient-from-position); --tw-gradient-to: rgb(185 28 28 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-800`; css `--tw-gradient-from: #991b1b var(--tw-gradient-from-position); --tw-gradient-to: rgb(153 27 27 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_800() -> Style {
    Style::new(css!("--tw-gradient-from: #991b1b var(--tw-gradient-from-position); --tw-gradient-to: rgb(153 27 27 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-900`; css `--tw-gradient-from: #7f1d1d var(--tw-gradient-from-position); --tw-gradient-to: rgb(127 29 29 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_900() -> Style {
    Style::new(css!("--tw-gradient-from: #7f1d1d var(--tw-gradient-from-position); --tw-gradient-to: rgb(127 29 29 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-red-950`; css `--tw-gradient-from: #450a0a var(--tw-gradient-from-position); --tw-gradient-to: rgb(69 10 10 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_red_950() -> Style {
    Style::new(css!("--tw-gradient-from: #450a0a var(--tw-gradient-from-position); --tw-gradient-to: rgb(69 10 10 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-50`; css `--tw-gradient-from: #fff7ed var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 247 237 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fff7ed var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 247 237 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-100`; css `--tw-gradient-from: #ffedd5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 237 213 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_100() -> Style {
    Style::new(css!("--tw-gradient-from: #ffedd5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 237 213 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-200`; css `--tw-gradient-from: #fed7aa var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 215 170 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_200() -> Style {
    Style::new(css!("--tw-gradient-from: #fed7aa var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 215 170 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-300`; css `--tw-gradient-from: #fdba74 var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 186 116 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_300() -> Style {
    Style::new(css!("--tw-gradient-from: #fdba74 var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 186 116 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-400`; css `--tw-gradient-from: #fb923c var(--tw-gradient-from-position); --tw-gradient-to: rgb(251 146 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_400() -> Style {
    Style::new(css!("--tw-gradient-from: #fb923c var(--tw-gradient-from-position); --tw-gradient-to: rgb(251 146 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-500`; css `--tw-gradient-from: #f97316 var(--tw-gradient-from-position); --tw-gradient-to: rgb(249 115 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_500() -> Style {
    Style::new(css!("--tw-gradient-from: #f97316 var(--tw-gradient-from-position); --tw-gradient-to: rgb(249 115 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-600`; css `--tw-gradient-from: #ea580c var(--tw-gradient-from-position); --tw-gradient-to: rgb(234 88 12 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_600() -> Style {
    Style::new(css!("--tw-gradient-from: #ea580c var(--tw-gradient-from-position); --tw-gradient-to: rgb(234 88 12 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-700`; css `--tw-gradient-from: #c2410c var(--tw-gradient-from-position); --tw-gradient-to: rgb(194 65 12 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_700() -> Style {
    Style::new(css!("--tw-gradient-from: #c2410c var(--tw-gradient-from-position); --tw-gradient-to: rgb(194 65 12 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-800`; css `--tw-gradient-from: #9a3412 var(--tw-gradient-from-position); --tw-gradient-to: rgb(154 52 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_800() -> Style {
    Style::new(css!("--tw-gradient-from: #9a3412 var(--tw-gradient-from-position); --tw-gradient-to: rgb(154 52 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-900`; css `--tw-gradient-from: #7c2d12 var(--tw-gradient-from-position); --tw-gradient-to: rgb(124 45 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_900() -> Style {
    Style::new(css!("--tw-gradient-from: #7c2d12 var(--tw-gradient-from-position); --tw-gradient-to: rgb(124 45 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-orange-950`; css `--tw-gradient-from: #431407 var(--tw-gradient-from-position); --tw-gradient-to: rgb(67 20 7 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_orange_950() -> Style {
    Style::new(css!("--tw-gradient-from: #431407 var(--tw-gradient-from-position); --tw-gradient-to: rgb(67 20 7 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-50`; css `--tw-gradient-from: #fffbeb var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 251 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fffbeb var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 251 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-100`; css `--tw-gradient-from: #fef3c7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 243 199 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_100() -> Style {
    Style::new(css!("--tw-gradient-from: #fef3c7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 243 199 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-200`; css `--tw-gradient-from: #fde68a var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 230 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_200() -> Style {
    Style::new(css!("--tw-gradient-from: #fde68a var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 230 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-300`; css `--tw-gradient-from: #fcd34d var(--tw-gradient-from-position); --tw-gradient-to: rgb(252 211 77 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_300() -> Style {
    Style::new(css!("--tw-gradient-from: #fcd34d var(--tw-gradient-from-position); --tw-gradient-to: rgb(252 211 77 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-400`; css `--tw-gradient-from: #fbbf24 var(--tw-gradient-from-position); --tw-gradient-to: rgb(251 191 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_400() -> Style {
    Style::new(css!("--tw-gradient-from: #fbbf24 var(--tw-gradient-from-position); --tw-gradient-to: rgb(251 191 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-500`; css `--tw-gradient-from: #f59e0b var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 158 11 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_500() -> Style {
    Style::new(css!("--tw-gradient-from: #f59e0b var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 158 11 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-600`; css `--tw-gradient-from: #d97706 var(--tw-gradient-from-position); --tw-gradient-to: rgb(217 119 6 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_600() -> Style {
    Style::new(css!("--tw-gradient-from: #d97706 var(--tw-gradient-from-position); --tw-gradient-to: rgb(217 119 6 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-700`; css `--tw-gradient-from: #b45309 var(--tw-gradient-from-position); --tw-gradient-to: rgb(180 83 9 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_700() -> Style {
    Style::new(css!("--tw-gradient-from: #b45309 var(--tw-gradient-from-position); --tw-gradient-to: rgb(180 83 9 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-800`; css `--tw-gradient-from: #92400e var(--tw-gradient-from-position); --tw-gradient-to: rgb(146 64 14 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_800() -> Style {
    Style::new(css!("--tw-gradient-from: #92400e var(--tw-gradient-from-position); --tw-gradient-to: rgb(146 64 14 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-900`; css `--tw-gradient-from: #78350f var(--tw-gradient-from-position); --tw-gradient-to: rgb(120 53 15 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_900() -> Style {
    Style::new(css!("--tw-gradient-from: #78350f var(--tw-gradient-from-position); --tw-gradient-to: rgb(120 53 15 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-amber-950`; css `--tw-gradient-from: #451a03 var(--tw-gradient-from-position); --tw-gradient-to: rgb(69 26 3 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_amber_950() -> Style {
    Style::new(css!("--tw-gradient-from: #451a03 var(--tw-gradient-from-position); --tw-gradient-to: rgb(69 26 3 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-50`; css `--tw-gradient-from: #fefce8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 252 232 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fefce8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 252 232 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-100`; css `--tw-gradient-from: #fef9c3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 249 195 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_100() -> Style {
    Style::new(css!("--tw-gradient-from: #fef9c3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 249 195 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-200`; css `--tw-gradient-from: #fef08a var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 240 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_200() -> Style {
    Style::new(css!("--tw-gradient-from: #fef08a var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 240 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-300`; css `--tw-gradient-from: #fde047 var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 224 71 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_300() -> Style {
    Style::new(css!("--tw-gradient-from: #fde047 var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 224 71 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-400`; css `--tw-gradient-from: #facc15 var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 204 21 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_400() -> Style {
    Style::new(css!("--tw-gradient-from: #facc15 var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 204 21 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-500`; css `--tw-gradient-from: #eab308 var(--tw-gradient-from-position); --tw-gradient-to: rgb(234 179 8 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_500() -> Style {
    Style::new(css!("--tw-gradient-from: #eab308 var(--tw-gradient-from-position); --tw-gradient-to: rgb(234 179 8 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-600`; css `--tw-gradient-from: #ca8a04 var(--tw-gradient-from-position); --tw-gradient-to: rgb(202 138 4 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_600() -> Style {
    Style::new(css!("--tw-gradient-from: #ca8a04 var(--tw-gradient-from-position); --tw-gradient-to: rgb(202 138 4 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-700`; css `--tw-gradient-from: #a16207 var(--tw-gradient-from-position); --tw-gradient-to: rgb(161 98 7 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_700() -> Style {
    Style::new(css!("--tw-gradient-from: #a16207 var(--tw-gradient-from-position); --tw-gradient-to: rgb(161 98 7 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-800`; css `--tw-gradient-from: #854d0e var(--tw-gradient-from-position); --tw-gradient-to: rgb(133 77 14 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_800() -> Style {
    Style::new(css!("--tw-gradient-from: #854d0e var(--tw-gradient-from-position); --tw-gradient-to: rgb(133 77 14 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-900`; css `--tw-gradient-from: #713f12 var(--tw-gradient-from-position); --tw-gradient-to: rgb(113 63 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_900() -> Style {
    Style::new(css!("--tw-gradient-from: #713f12 var(--tw-gradient-from-position); --tw-gradient-to: rgb(113 63 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-yellow-950`; css `--tw-gradient-from: #422006 var(--tw-gradient-from-position); --tw-gradient-to: rgb(66 32 6 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_yellow_950() -> Style {
    Style::new(css!("--tw-gradient-from: #422006 var(--tw-gradient-from-position); --tw-gradient-to: rgb(66 32 6 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-50`; css `--tw-gradient-from: #f7fee7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(247 254 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_50() -> Style {
    Style::new(css!("--tw-gradient-from: #f7fee7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(247 254 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-100`; css `--tw-gradient-from: #ecfccb var(--tw-gradient-from-position); --tw-gradient-to: rgb(236 252 203 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_100() -> Style {
    Style::new(css!("--tw-gradient-from: #ecfccb var(--tw-gradient-from-position); --tw-gradient-to: rgb(236 252 203 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-200`; css `--tw-gradient-from: #d9f99d var(--tw-gradient-from-position); --tw-gradient-to: rgb(217 249 157 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_200() -> Style {
    Style::new(css!("--tw-gradient-from: #d9f99d var(--tw-gradient-from-position); --tw-gradient-to: rgb(217 249 157 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-300`; css `--tw-gradient-from: #bef264 var(--tw-gradient-from-position); --tw-gradient-to: rgb(190 242 100 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_300() -> Style {
    Style::new(css!("--tw-gradient-from: #bef264 var(--tw-gradient-from-position); --tw-gradient-to: rgb(190 242 100 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-400`; css `--tw-gradient-from: #a3e635 var(--tw-gradient-from-position); --tw-gradient-to: rgb(163 230 53 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_400() -> Style {
    Style::new(css!("--tw-gradient-from: #a3e635 var(--tw-gradient-from-position); --tw-gradient-to: rgb(163 230 53 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-500`; css `--tw-gradient-from: #84cc16 var(--tw-gradient-from-position); --tw-gradient-to: rgb(132 204 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_500() -> Style {
    Style::new(css!("--tw-gradient-from: #84cc16 var(--tw-gradient-from-position); --tw-gradient-to: rgb(132 204 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-600`; css `--tw-gradient-from: #65a30d var(--tw-gradient-from-position); --tw-gradient-to: rgb(101 163 13 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_600() -> Style {
    Style::new(css!("--tw-gradient-from: #65a30d var(--tw-gradient-from-position); --tw-gradient-to: rgb(101 163 13 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-700`; css `--tw-gradient-from: #4d7c0f var(--tw-gradient-from-position); --tw-gradient-to: rgb(77 124 15 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_700() -> Style {
    Style::new(css!("--tw-gradient-from: #4d7c0f var(--tw-gradient-from-position); --tw-gradient-to: rgb(77 124 15 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-800`; css `--tw-gradient-from: #3f6212 var(--tw-gradient-from-position); --tw-gradient-to: rgb(63 98 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_800() -> Style {
    Style::new(css!("--tw-gradient-from: #3f6212 var(--tw-gradient-from-position); --tw-gradient-to: rgb(63 98 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-900`; css `--tw-gradient-from: #365314 var(--tw-gradient-from-position); --tw-gradient-to: rgb(54 83 20 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_900() -> Style {
    Style::new(css!("--tw-gradient-from: #365314 var(--tw-gradient-from-position); --tw-gradient-to: rgb(54 83 20 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-lime-950`; css `--tw-gradient-from: #1a2e05 var(--tw-gradient-from-position); --tw-gradient-to: rgb(26 46 5 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_lime_950() -> Style {
    Style::new(css!("--tw-gradient-from: #1a2e05 var(--tw-gradient-from-position); --tw-gradient-to: rgb(26 46 5 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-50`; css `--tw-gradient-from: #f0fdf4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(240 253 244 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_50() -> Style {
    Style::new(css!("--tw-gradient-from: #f0fdf4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(240 253 244 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-100`; css `--tw-gradient-from: #dcfce7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(220 252 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_100() -> Style {
    Style::new(css!("--tw-gradient-from: #dcfce7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(220 252 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-200`; css `--tw-gradient-from: #bbf7d0 var(--tw-gradient-from-position); --tw-gradient-to: rgb(187 247 208 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_200() -> Style {
    Style::new(css!("--tw-gradient-from: #bbf7d0 var(--tw-gradient-from-position); --tw-gradient-to: rgb(187 247 208 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-300`; css `--tw-gradient-from: #86efac var(--tw-gradient-from-position); --tw-gradient-to: rgb(134 239 172 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_300() -> Style {
    Style::new(css!("--tw-gradient-from: #86efac var(--tw-gradient-from-position); --tw-gradient-to: rgb(134 239 172 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-400`; css `--tw-gradient-from: #4ade80 var(--tw-gradient-from-position); --tw-gradient-to: rgb(74 222 128 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_400() -> Style {
    Style::new(css!("--tw-gradient-from: #4ade80 var(--tw-gradient-from-position); --tw-gradient-to: rgb(74 222 128 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-500`; css `--tw-gradient-from: #22c55e var(--tw-gradient-from-position); --tw-gradient-to: rgb(34 197 94 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_500() -> Style {
    Style::new(css!("--tw-gradient-from: #22c55e var(--tw-gradient-from-position); --tw-gradient-to: rgb(34 197 94 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-600`; css `--tw-gradient-from: #16a34a var(--tw-gradient-from-position); --tw-gradient-to: rgb(22 163 74 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_600() -> Style {
    Style::new(css!("--tw-gradient-from: #16a34a var(--tw-gradient-from-position); --tw-gradient-to: rgb(22 163 74 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-700`; css `--tw-gradient-from: #15803d var(--tw-gradient-from-position); --tw-gradient-to: rgb(21 128 61 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_700() -> Style {
    Style::new(css!("--tw-gradient-from: #15803d var(--tw-gradient-from-position); --tw-gradient-to: rgb(21 128 61 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-800`; css `--tw-gradient-from: #166534 var(--tw-gradient-from-position); --tw-gradient-to: rgb(22 101 52 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_800() -> Style {
    Style::new(css!("--tw-gradient-from: #166534 var(--tw-gradient-from-position); --tw-gradient-to: rgb(22 101 52 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-900`; css `--tw-gradient-from: #14532d var(--tw-gradient-from-position); --tw-gradient-to: rgb(20 83 45 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_900() -> Style {
    Style::new(css!("--tw-gradient-from: #14532d var(--tw-gradient-from-position); --tw-gradient-to: rgb(20 83 45 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-green-950`; css `--tw-gradient-from: #052e16 var(--tw-gradient-from-position); --tw-gradient-to: rgb(5 46 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_green_950() -> Style {
    Style::new(css!("--tw-gradient-from: #052e16 var(--tw-gradient-from-position); --tw-gradient-to: rgb(5 46 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-50`; css `--tw-gradient-from: #ecfdf5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(236 253 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_50() -> Style {
    Style::new(css!("--tw-gradient-from: #ecfdf5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(236 253 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-100`; css `--tw-gradient-from: #d1fae5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(209 250 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_100() -> Style {
    Style::new(css!("--tw-gradient-from: #d1fae5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(209 250 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-200`; css `--tw-gradient-from: #a7f3d0 var(--tw-gradient-from-position); --tw-gradient-to: rgb(167 243 208 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_200() -> Style {
    Style::new(css!("--tw-gradient-from: #a7f3d0 var(--tw-gradient-from-position); --tw-gradient-to: rgb(167 243 208 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-300`; css `--tw-gradient-from: #6ee7b7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(110 231 183 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_300() -> Style {
    Style::new(css!("--tw-gradient-from: #6ee7b7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(110 231 183 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-400`; css `--tw-gradient-from: #34d399 var(--tw-gradient-from-position); --tw-gradient-to: rgb(52 211 153 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_400() -> Style {
    Style::new(css!("--tw-gradient-from: #34d399 var(--tw-gradient-from-position); --tw-gradient-to: rgb(52 211 153 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-500`; css `--tw-gradient-from: #10b981 var(--tw-gradient-from-position); --tw-gradient-to: rgb(16 185 129 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_500() -> Style {
    Style::new(css!("--tw-gradient-from: #10b981 var(--tw-gradient-from-position); --tw-gradient-to: rgb(16 185 129 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-600`; css `--tw-gradient-from: #059669 var(--tw-gradient-from-position); --tw-gradient-to: rgb(5 150 105 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_600() -> Style {
    Style::new(css!("--tw-gradient-from: #059669 var(--tw-gradient-from-position); --tw-gradient-to: rgb(5 150 105 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-700`; css `--tw-gradient-from: #047857 var(--tw-gradient-from-position); --tw-gradient-to: rgb(4 120 87 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_700() -> Style {
    Style::new(css!("--tw-gradient-from: #047857 var(--tw-gradient-from-position); --tw-gradient-to: rgb(4 120 87 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-800`; css `--tw-gradient-from: #065f46 var(--tw-gradient-from-position); --tw-gradient-to: rgb(6 95 70 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_800() -> Style {
    Style::new(css!("--tw-gradient-from: #065f46 var(--tw-gradient-from-position); --tw-gradient-to: rgb(6 95 70 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-900`; css `--tw-gradient-from: #064e3b var(--tw-gradient-from-position); --tw-gradient-to: rgb(6 78 59 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_900() -> Style {
    Style::new(css!("--tw-gradient-from: #064e3b var(--tw-gradient-from-position); --tw-gradient-to: rgb(6 78 59 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-emerald-950`; css `--tw-gradient-from: #022c22 var(--tw-gradient-from-position); --tw-gradient-to: rgb(2 44 34 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_emerald_950() -> Style {
    Style::new(css!("--tw-gradient-from: #022c22 var(--tw-gradient-from-position); --tw-gradient-to: rgb(2 44 34 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-50`; css `--tw-gradient-from: #f0fdfa var(--tw-gradient-from-position); --tw-gradient-to: rgb(240 253 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_50() -> Style {
    Style::new(css!("--tw-gradient-from: #f0fdfa var(--tw-gradient-from-position); --tw-gradient-to: rgb(240 253 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-100`; css `--tw-gradient-from: #ccfbf1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(204 251 241 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_100() -> Style {
    Style::new(css!("--tw-gradient-from: #ccfbf1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(204 251 241 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-200`; css `--tw-gradient-from: #99f6e4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(153 246 228 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_200() -> Style {
    Style::new(css!("--tw-gradient-from: #99f6e4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(153 246 228 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-300`; css `--tw-gradient-from: #5eead4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(94 234 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_300() -> Style {
    Style::new(css!("--tw-gradient-from: #5eead4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(94 234 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-400`; css `--tw-gradient-from: #2dd4bf var(--tw-gradient-from-position); --tw-gradient-to: rgb(45 212 191 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_400() -> Style {
    Style::new(css!("--tw-gradient-from: #2dd4bf var(--tw-gradient-from-position); --tw-gradient-to: rgb(45 212 191 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-500`; css `--tw-gradient-from: #14b8a6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(20 184 166 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_500() -> Style {
    Style::new(css!("--tw-gradient-from: #14b8a6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(20 184 166 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-600`; css `--tw-gradient-from: #0d9488 var(--tw-gradient-from-position); --tw-gradient-to: rgb(13 148 136 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_600() -> Style {
    Style::new(css!("--tw-gradient-from: #0d9488 var(--tw-gradient-from-position); --tw-gradient-to: rgb(13 148 136 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-700`; css `--tw-gradient-from: #0f766e var(--tw-gradient-from-position); --tw-gradient-to: rgb(15 118 110 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_700() -> Style {
    Style::new(css!("--tw-gradient-from: #0f766e var(--tw-gradient-from-position); --tw-gradient-to: rgb(15 118 110 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-800`; css `--tw-gradient-from: #115e59 var(--tw-gradient-from-position); --tw-gradient-to: rgb(17 94 89 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_800() -> Style {
    Style::new(css!("--tw-gradient-from: #115e59 var(--tw-gradient-from-position); --tw-gradient-to: rgb(17 94 89 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-900`; css `--tw-gradient-from: #134e4a var(--tw-gradient-from-position); --tw-gradient-to: rgb(19 78 74 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_900() -> Style {
    Style::new(css!("--tw-gradient-from: #134e4a var(--tw-gradient-from-position); --tw-gradient-to: rgb(19 78 74 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-teal-950`; css `--tw-gradient-from: #042f2e var(--tw-gradient-from-position); --tw-gradient-to: rgb(4 47 46 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_teal_950() -> Style {
    Style::new(css!("--tw-gradient-from: #042f2e var(--tw-gradient-from-position); --tw-gradient-to: rgb(4 47 46 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-50`; css `--tw-gradient-from: #ecfeff var(--tw-gradient-from-position); --tw-gradient-to: rgb(236 254 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_50() -> Style {
    Style::new(css!("--tw-gradient-from: #ecfeff var(--tw-gradient-from-position); --tw-gradient-to: rgb(236 254 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-100`; css `--tw-gradient-from: #cffafe var(--tw-gradient-from-position); --tw-gradient-to: rgb(207 250 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_100() -> Style {
    Style::new(css!("--tw-gradient-from: #cffafe var(--tw-gradient-from-position); --tw-gradient-to: rgb(207 250 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-200`; css `--tw-gradient-from: #a5f3fc var(--tw-gradient-from-position); --tw-gradient-to: rgb(165 243 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_200() -> Style {
    Style::new(css!("--tw-gradient-from: #a5f3fc var(--tw-gradient-from-position); --tw-gradient-to: rgb(165 243 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-300`; css `--tw-gradient-from: #67e8f9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(103 232 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_300() -> Style {
    Style::new(css!("--tw-gradient-from: #67e8f9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(103 232 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-400`; css `--tw-gradient-from: #22d3ee var(--tw-gradient-from-position); --tw-gradient-to: rgb(34 211 238 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_400() -> Style {
    Style::new(css!("--tw-gradient-from: #22d3ee var(--tw-gradient-from-position); --tw-gradient-to: rgb(34 211 238 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-500`; css `--tw-gradient-from: #06b6d4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(6 182 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_500() -> Style {
    Style::new(css!("--tw-gradient-from: #06b6d4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(6 182 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-600`; css `--tw-gradient-from: #0891b2 var(--tw-gradient-from-position); --tw-gradient-to: rgb(8 145 178 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_600() -> Style {
    Style::new(css!("--tw-gradient-from: #0891b2 var(--tw-gradient-from-position); --tw-gradient-to: rgb(8 145 178 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-700`; css `--tw-gradient-from: #0e7490 var(--tw-gradient-from-position); --tw-gradient-to: rgb(14 116 144 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_700() -> Style {
    Style::new(css!("--tw-gradient-from: #0e7490 var(--tw-gradient-from-position); --tw-gradient-to: rgb(14 116 144 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-800`; css `--tw-gradient-from: #155e75 var(--tw-gradient-from-position); --tw-gradient-to: rgb(21 94 117 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_800() -> Style {
    Style::new(css!("--tw-gradient-from: #155e75 var(--tw-gradient-from-position); --tw-gradient-to: rgb(21 94 117 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-900`; css `--tw-gradient-from: #164e63 var(--tw-gradient-from-position); --tw-gradient-to: rgb(22 78 99 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_900() -> Style {
    Style::new(css!("--tw-gradient-from: #164e63 var(--tw-gradient-from-position); --tw-gradient-to: rgb(22 78 99 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-cyan-950`; css `--tw-gradient-from: #083344 var(--tw-gradient-from-position); --tw-gradient-to: rgb(8 51 68 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_cyan_950() -> Style {
    Style::new(css!("--tw-gradient-from: #083344 var(--tw-gradient-from-position); --tw-gradient-to: rgb(8 51 68 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-50`; css `--tw-gradient-from: #f0f9ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(240 249 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_50() -> Style {
    Style::new(css!("--tw-gradient-from: #f0f9ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(240 249 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-100`; css `--tw-gradient-from: #e0f2fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(224 242 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_100() -> Style {
    Style::new(css!("--tw-gradient-from: #e0f2fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(224 242 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-200`; css `--tw-gradient-from: #bae6fd var(--tw-gradient-from-position); --tw-gradient-to: rgb(186 230 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_200() -> Style {
    Style::new(css!("--tw-gradient-from: #bae6fd var(--tw-gradient-from-position); --tw-gradient-to: rgb(186 230 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-300`; css `--tw-gradient-from: #7dd3fc var(--tw-gradient-from-position); --tw-gradient-to: rgb(125 211 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_300() -> Style {
    Style::new(css!("--tw-gradient-from: #7dd3fc var(--tw-gradient-from-position); --tw-gradient-to: rgb(125 211 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-400`; css `--tw-gradient-from: #38bdf8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(56 189 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_400() -> Style {
    Style::new(css!("--tw-gradient-from: #38bdf8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(56 189 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-500`; css `--tw-gradient-from: #0ea5e9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(14 165 233 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_500() -> Style {
    Style::new(css!("--tw-gradient-from: #0ea5e9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(14 165 233 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-600`; css `--tw-gradient-from: #0284c7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(2 132 199 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_600() -> Style {
    Style::new(css!("--tw-gradient-from: #0284c7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(2 132 199 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-700`; css `--tw-gradient-from: #0369a1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(3 105 161 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_700() -> Style {
    Style::new(css!("--tw-gradient-from: #0369a1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(3 105 161 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-800`; css `--tw-gradient-from: #075985 var(--tw-gradient-from-position); --tw-gradient-to: rgb(7 89 133 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_800() -> Style {
    Style::new(css!("--tw-gradient-from: #075985 var(--tw-gradient-from-position); --tw-gradient-to: rgb(7 89 133 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-900`; css `--tw-gradient-from: #0c4a6e var(--tw-gradient-from-position); --tw-gradient-to: rgb(12 74 110 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_900() -> Style {
    Style::new(css!("--tw-gradient-from: #0c4a6e var(--tw-gradient-from-position); --tw-gradient-to: rgb(12 74 110 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-sky-950`; css `--tw-gradient-from: #082f49 var(--tw-gradient-from-position); --tw-gradient-to: rgb(8 47 73 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_sky_950() -> Style {
    Style::new(css!("--tw-gradient-from: #082f49 var(--tw-gradient-from-position); --tw-gradient-to: rgb(8 47 73 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-50`; css `--tw-gradient-from: #eff6ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(239 246 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_50() -> Style {
    Style::new(css!("--tw-gradient-from: #eff6ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(239 246 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-100`; css `--tw-gradient-from: #dbeafe var(--tw-gradient-from-position); --tw-gradient-to: rgb(219 234 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_100() -> Style {
    Style::new(css!("--tw-gradient-from: #dbeafe var(--tw-gradient-from-position); --tw-gradient-to: rgb(219 234 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-200`; css `--tw-gradient-from: #bfdbfe var(--tw-gradient-from-position); --tw-gradient-to: rgb(191 219 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_200() -> Style {
    Style::new(css!("--tw-gradient-from: #bfdbfe var(--tw-gradient-from-position); --tw-gradient-to: rgb(191 219 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-300`; css `--tw-gradient-from: #93c5fd var(--tw-gradient-from-position); --tw-gradient-to: rgb(147 197 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_300() -> Style {
    Style::new(css!("--tw-gradient-from: #93c5fd var(--tw-gradient-from-position); --tw-gradient-to: rgb(147 197 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-400`; css `--tw-gradient-from: #60a5fa var(--tw-gradient-from-position); --tw-gradient-to: rgb(96 165 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_400() -> Style {
    Style::new(css!("--tw-gradient-from: #60a5fa var(--tw-gradient-from-position); --tw-gradient-to: rgb(96 165 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-500`; css `--tw-gradient-from: #3b82f6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(59 130 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_500() -> Style {
    Style::new(css!("--tw-gradient-from: #3b82f6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(59 130 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-600`; css `--tw-gradient-from: #2563eb var(--tw-gradient-from-position); --tw-gradient-to: rgb(37 99 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_600() -> Style {
    Style::new(css!("--tw-gradient-from: #2563eb var(--tw-gradient-from-position); --tw-gradient-to: rgb(37 99 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-700`; css `--tw-gradient-from: #1d4ed8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(29 78 216 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_700() -> Style {
    Style::new(css!("--tw-gradient-from: #1d4ed8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(29 78 216 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-800`; css `--tw-gradient-from: #1e40af var(--tw-gradient-from-position); --tw-gradient-to: rgb(30 64 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_800() -> Style {
    Style::new(css!("--tw-gradient-from: #1e40af var(--tw-gradient-from-position); --tw-gradient-to: rgb(30 64 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-900`; css `--tw-gradient-from: #1e3a8a var(--tw-gradient-from-position); --tw-gradient-to: rgb(30 58 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_900() -> Style {
    Style::new(css!("--tw-gradient-from: #1e3a8a var(--tw-gradient-from-position); --tw-gradient-to: rgb(30 58 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-blue-950`; css `--tw-gradient-from: #172554 var(--tw-gradient-from-position); --tw-gradient-to: rgb(23 37 84 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_blue_950() -> Style {
    Style::new(css!("--tw-gradient-from: #172554 var(--tw-gradient-from-position); --tw-gradient-to: rgb(23 37 84 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-50`; css `--tw-gradient-from: #eef2ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(238 242 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_50() -> Style {
    Style::new(css!("--tw-gradient-from: #eef2ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(238 242 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-100`; css `--tw-gradient-from: #e0e7ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(224 231 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_100() -> Style {
    Style::new(css!("--tw-gradient-from: #e0e7ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(224 231 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-200`; css `--tw-gradient-from: #c7d2fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(199 210 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_200() -> Style {
    Style::new(css!("--tw-gradient-from: #c7d2fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(199 210 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-300`; css `--tw-gradient-from: #a5b4fc var(--tw-gradient-from-position); --tw-gradient-to: rgb(165 180 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_300() -> Style {
    Style::new(css!("--tw-gradient-from: #a5b4fc var(--tw-gradient-from-position); --tw-gradient-to: rgb(165 180 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-400`; css `--tw-gradient-from: #818cf8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(129 140 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_400() -> Style {
    Style::new(css!("--tw-gradient-from: #818cf8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(129 140 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-500`; css `--tw-gradient-from: #6366f1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(99 102 241 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_500() -> Style {
    Style::new(css!("--tw-gradient-from: #6366f1 var(--tw-gradient-from-position); --tw-gradient-to: rgb(99 102 241 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-600`; css `--tw-gradient-from: #4f46e5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(79 70 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_600() -> Style {
    Style::new(css!("--tw-gradient-from: #4f46e5 var(--tw-gradient-from-position); --tw-gradient-to: rgb(79 70 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-700`; css `--tw-gradient-from: #4338ca var(--tw-gradient-from-position); --tw-gradient-to: rgb(67 56 202 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_700() -> Style {
    Style::new(css!("--tw-gradient-from: #4338ca var(--tw-gradient-from-position); --tw-gradient-to: rgb(67 56 202 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-800`; css `--tw-gradient-from: #3730a3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(55 48 163 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_800() -> Style {
    Style::new(css!("--tw-gradient-from: #3730a3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(55 48 163 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-900`; css `--tw-gradient-from: #312e81 var(--tw-gradient-from-position); --tw-gradient-to: rgb(49 46 129 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_900() -> Style {
    Style::new(css!("--tw-gradient-from: #312e81 var(--tw-gradient-from-position); --tw-gradient-to: rgb(49 46 129 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-indigo-950`; css `--tw-gradient-from: #1e1b4b var(--tw-gradient-from-position); --tw-gradient-to: rgb(30 27 75 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_indigo_950() -> Style {
    Style::new(css!("--tw-gradient-from: #1e1b4b var(--tw-gradient-from-position); --tw-gradient-to: rgb(30 27 75 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-50`; css `--tw-gradient-from: #f5f3ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 243 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_50() -> Style {
    Style::new(css!("--tw-gradient-from: #f5f3ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 243 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-100`; css `--tw-gradient-from: #ede9fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(237 233 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_100() -> Style {
    Style::new(css!("--tw-gradient-from: #ede9fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(237 233 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-200`; css `--tw-gradient-from: #ddd6fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(221 214 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_200() -> Style {
    Style::new(css!("--tw-gradient-from: #ddd6fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(221 214 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-300`; css `--tw-gradient-from: #c4b5fd var(--tw-gradient-from-position); --tw-gradient-to: rgb(196 181 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_300() -> Style {
    Style::new(css!("--tw-gradient-from: #c4b5fd var(--tw-gradient-from-position); --tw-gradient-to: rgb(196 181 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-400`; css `--tw-gradient-from: #a78bfa var(--tw-gradient-from-position); --tw-gradient-to: rgb(167 139 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_400() -> Style {
    Style::new(css!("--tw-gradient-from: #a78bfa var(--tw-gradient-from-position); --tw-gradient-to: rgb(167 139 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-500`; css `--tw-gradient-from: #8b5cf6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(139 92 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_500() -> Style {
    Style::new(css!("--tw-gradient-from: #8b5cf6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(139 92 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-600`; css `--tw-gradient-from: #7c3aed var(--tw-gradient-from-position); --tw-gradient-to: rgb(124 58 237 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_600() -> Style {
    Style::new(css!("--tw-gradient-from: #7c3aed var(--tw-gradient-from-position); --tw-gradient-to: rgb(124 58 237 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-700`; css `--tw-gradient-from: #6d28d9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(109 40 217 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_700() -> Style {
    Style::new(css!("--tw-gradient-from: #6d28d9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(109 40 217 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-800`; css `--tw-gradient-from: #5b21b6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(91 33 182 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_800() -> Style {
    Style::new(css!("--tw-gradient-from: #5b21b6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(91 33 182 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-900`; css `--tw-gradient-from: #4c1d95 var(--tw-gradient-from-position); --tw-gradient-to: rgb(76 29 149 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_900() -> Style {
    Style::new(css!("--tw-gradient-from: #4c1d95 var(--tw-gradient-from-position); --tw-gradient-to: rgb(76 29 149 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-violet-950`; css `--tw-gradient-from: #2e1065 var(--tw-gradient-from-position); --tw-gradient-to: rgb(46 16 101 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_violet_950() -> Style {
    Style::new(css!("--tw-gradient-from: #2e1065 var(--tw-gradient-from-position); --tw-gradient-to: rgb(46 16 101 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-50`; css `--tw-gradient-from: #faf5ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 245 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_50() -> Style {
    Style::new(css!("--tw-gradient-from: #faf5ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 245 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-100`; css `--tw-gradient-from: #f3e8ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(243 232 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_100() -> Style {
    Style::new(css!("--tw-gradient-from: #f3e8ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(243 232 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-200`; css `--tw-gradient-from: #e9d5ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(233 213 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_200() -> Style {
    Style::new(css!("--tw-gradient-from: #e9d5ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(233 213 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-300`; css `--tw-gradient-from: #d8b4fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(216 180 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_300() -> Style {
    Style::new(css!("--tw-gradient-from: #d8b4fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(216 180 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-400`; css `--tw-gradient-from: #c084fc var(--tw-gradient-from-position); --tw-gradient-to: rgb(192 132 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_400() -> Style {
    Style::new(css!("--tw-gradient-from: #c084fc var(--tw-gradient-from-position); --tw-gradient-to: rgb(192 132 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-500`; css `--tw-gradient-from: #a855f7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(168 85 247 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_500() -> Style {
    Style::new(css!("--tw-gradient-from: #a855f7 var(--tw-gradient-from-position); --tw-gradient-to: rgb(168 85 247 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-600`; css `--tw-gradient-from: #9333ea var(--tw-gradient-from-position); --tw-gradient-to: rgb(147 51 234 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_600() -> Style {
    Style::new(css!("--tw-gradient-from: #9333ea var(--tw-gradient-from-position); --tw-gradient-to: rgb(147 51 234 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-700`; css `--tw-gradient-from: #7e22ce var(--tw-gradient-from-position); --tw-gradient-to: rgb(126 34 206 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_700() -> Style {
    Style::new(css!("--tw-gradient-from: #7e22ce var(--tw-gradient-from-position); --tw-gradient-to: rgb(126 34 206 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-800`; css `--tw-gradient-from: #6b21a8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(107 33 168 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_800() -> Style {
    Style::new(css!("--tw-gradient-from: #6b21a8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(107 33 168 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-900`; css `--tw-gradient-from: #581c87 var(--tw-gradient-from-position); --tw-gradient-to: rgb(88 28 135 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_900() -> Style {
    Style::new(css!("--tw-gradient-from: #581c87 var(--tw-gradient-from-position); --tw-gradient-to: rgb(88 28 135 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-purple-950`; css `--tw-gradient-from: #3b0764 var(--tw-gradient-from-position); --tw-gradient-to: rgb(59 7 100 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_purple_950() -> Style {
    Style::new(css!("--tw-gradient-from: #3b0764 var(--tw-gradient-from-position); --tw-gradient-to: rgb(59 7 100 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-50`; css `--tw-gradient-from: #fdf4ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 244 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fdf4ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 244 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-100`; css `--tw-gradient-from: #fae8ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 232 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_100() -> Style {
    Style::new(css!("--tw-gradient-from: #fae8ff var(--tw-gradient-from-position); --tw-gradient-to: rgb(250 232 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-200`; css `--tw-gradient-from: #f5d0fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 208 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_200() -> Style {
    Style::new(css!("--tw-gradient-from: #f5d0fe var(--tw-gradient-from-position); --tw-gradient-to: rgb(245 208 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-300`; css `--tw-gradient-from: #f0abfc var(--tw-gradient-from-position); --tw-gradient-to: rgb(240 171 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_300() -> Style {
    Style::new(css!("--tw-gradient-from: #f0abfc var(--tw-gradient-from-position); --tw-gradient-to: rgb(240 171 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-400`; css `--tw-gradient-from: #e879f9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(232 121 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_400() -> Style {
    Style::new(css!("--tw-gradient-from: #e879f9 var(--tw-gradient-from-position); --tw-gradient-to: rgb(232 121 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-500`; css `--tw-gradient-from: #d946ef var(--tw-gradient-from-position); --tw-gradient-to: rgb(217 70 239 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_500() -> Style {
    Style::new(css!("--tw-gradient-from: #d946ef var(--tw-gradient-from-position); --tw-gradient-to: rgb(217 70 239 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-600`; css `--tw-gradient-from: #c026d3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(192 38 211 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_600() -> Style {
    Style::new(css!("--tw-gradient-from: #c026d3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(192 38 211 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-700`; css `--tw-gradient-from: #a21caf var(--tw-gradient-from-position); --tw-gradient-to: rgb(162 28 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_700() -> Style {
    Style::new(css!("--tw-gradient-from: #a21caf var(--tw-gradient-from-position); --tw-gradient-to: rgb(162 28 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-800`; css `--tw-gradient-from: #86198f var(--tw-gradient-from-position); --tw-gradient-to: rgb(134 25 143 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_800() -> Style {
    Style::new(css!("--tw-gradient-from: #86198f var(--tw-gradient-from-position); --tw-gradient-to: rgb(134 25 143 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-900`; css `--tw-gradient-from: #701a75 var(--tw-gradient-from-position); --tw-gradient-to: rgb(112 26 117 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_900() -> Style {
    Style::new(css!("--tw-gradient-from: #701a75 var(--tw-gradient-from-position); --tw-gradient-to: rgb(112 26 117 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-fuchsia-950`; css `--tw-gradient-from: #4a044e var(--tw-gradient-from-position); --tw-gradient-to: rgb(74 4 78 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_fuchsia_950() -> Style {
    Style::new(css!("--tw-gradient-from: #4a044e var(--tw-gradient-from-position); --tw-gradient-to: rgb(74 4 78 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-50`; css `--tw-gradient-from: #fdf2f8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 242 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fdf2f8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 242 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-100`; css `--tw-gradient-from: #fce7f3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(252 231 243 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_100() -> Style {
    Style::new(css!("--tw-gradient-from: #fce7f3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(252 231 243 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-200`; css `--tw-gradient-from: #fbcfe8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(251 207 232 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_200() -> Style {
    Style::new(css!("--tw-gradient-from: #fbcfe8 var(--tw-gradient-from-position); --tw-gradient-to: rgb(251 207 232 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-300`; css `--tw-gradient-from: #f9a8d4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(249 168 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_300() -> Style {
    Style::new(css!("--tw-gradient-from: #f9a8d4 var(--tw-gradient-from-position); --tw-gradient-to: rgb(249 168 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-400`; css `--tw-gradient-from: #f472b6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(244 114 182 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_400() -> Style {
    Style::new(css!("--tw-gradient-from: #f472b6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(244 114 182 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-500`; css `--tw-gradient-from: #ec4899 var(--tw-gradient-from-position); --tw-gradient-to: rgb(236 72 153 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_500() -> Style {
    Style::new(css!("--tw-gradient-from: #ec4899 var(--tw-gradient-from-position); --tw-gradient-to: rgb(236 72 153 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-600`; css `--tw-gradient-from: #db2777 var(--tw-gradient-from-position); --tw-gradient-to: rgb(219 39 119 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_600() -> Style {
    Style::new(css!("--tw-gradient-from: #db2777 var(--tw-gradient-from-position); --tw-gradient-to: rgb(219 39 119 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-700`; css `--tw-gradient-from: #be185d var(--tw-gradient-from-position); --tw-gradient-to: rgb(190 24 93 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_700() -> Style {
    Style::new(css!("--tw-gradient-from: #be185d var(--tw-gradient-from-position); --tw-gradient-to: rgb(190 24 93 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-800`; css `--tw-gradient-from: #9d174d var(--tw-gradient-from-position); --tw-gradient-to: rgb(157 23 77 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_800() -> Style {
    Style::new(css!("--tw-gradient-from: #9d174d var(--tw-gradient-from-position); --tw-gradient-to: rgb(157 23 77 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-900`; css `--tw-gradient-from: #831843 var(--tw-gradient-from-position); --tw-gradient-to: rgb(131 24 67 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_900() -> Style {
    Style::new(css!("--tw-gradient-from: #831843 var(--tw-gradient-from-position); --tw-gradient-to: rgb(131 24 67 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-pink-950`; css `--tw-gradient-from: #500724 var(--tw-gradient-from-position); --tw-gradient-to: rgb(80 7 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_pink_950() -> Style {
    Style::new(css!("--tw-gradient-from: #500724 var(--tw-gradient-from-position); --tw-gradient-to: rgb(80 7 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-50`; css `--tw-gradient-from: #fff1f2 var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 241 242 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_50() -> Style {
    Style::new(css!("--tw-gradient-from: #fff1f2 var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 241 242 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-100`; css `--tw-gradient-from: #ffe4e6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 228 230 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_100() -> Style {
    Style::new(css!("--tw-gradient-from: #ffe4e6 var(--tw-gradient-from-position); --tw-gradient-to: rgb(255 228 230 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-200`; css `--tw-gradient-from: #fecdd3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 205 211 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_200() -> Style {
    Style::new(css!("--tw-gradient-from: #fecdd3 var(--tw-gradient-from-position); --tw-gradient-to: rgb(254 205 211 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-300`; css `--tw-gradient-from: #fda4af var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 164 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_300() -> Style {
    Style::new(css!("--tw-gradient-from: #fda4af var(--tw-gradient-from-position); --tw-gradient-to: rgb(253 164 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-400`; css `--tw-gradient-from: #fb7185 var(--tw-gradient-from-position); --tw-gradient-to: rgb(251 113 133 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_400() -> Style {
    Style::new(css!("--tw-gradient-from: #fb7185 var(--tw-gradient-from-position); --tw-gradient-to: rgb(251 113 133 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-500`; css `--tw-gradient-from: #f43f5e var(--tw-gradient-from-position); --tw-gradient-to: rgb(244 63 94 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_500() -> Style {
    Style::new(css!("--tw-gradient-from: #f43f5e var(--tw-gradient-from-position); --tw-gradient-to: rgb(244 63 94 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-600`; css `--tw-gradient-from: #e11d48 var(--tw-gradient-from-position); --tw-gradient-to: rgb(225 29 72 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_600() -> Style {
    Style::new(css!("--tw-gradient-from: #e11d48 var(--tw-gradient-from-position); --tw-gradient-to: rgb(225 29 72 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-700`; css `--tw-gradient-from: #be123c var(--tw-gradient-from-position); --tw-gradient-to: rgb(190 18 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_700() -> Style {
    Style::new(css!("--tw-gradient-from: #be123c var(--tw-gradient-from-position); --tw-gradient-to: rgb(190 18 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-800`; css `--tw-gradient-from: #9f1239 var(--tw-gradient-from-position); --tw-gradient-to: rgb(159 18 57 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_800() -> Style {
    Style::new(css!("--tw-gradient-from: #9f1239 var(--tw-gradient-from-position); --tw-gradient-to: rgb(159 18 57 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-900`; css `--tw-gradient-from: #881337 var(--tw-gradient-from-position); --tw-gradient-to: rgb(136 19 55 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_900() -> Style {
    Style::new(css!("--tw-gradient-from: #881337 var(--tw-gradient-from-position); --tw-gradient-to: rgb(136 19 55 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-rose-950`; css `--tw-gradient-from: #4c0519 var(--tw-gradient-from-position); --tw-gradient-to: rgb(76 5 25 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);`
pub fn from_rose_950() -> Style {
    Style::new(css!("--tw-gradient-from: #4c0519 var(--tw-gradient-from-position); --tw-gradient-to: rgb(76 5 25 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `from-0%`; css `--tw-gradient-from-position: 0%;`
pub fn from_0p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 0%;")).unwrap()
}

/// Tailwind `from-5%`; css `--tw-gradient-from-position: 5%;`
pub fn from_5p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 5%;")).unwrap()
}

/// Tailwind `from-10%`; css `--tw-gradient-from-position: 10%;`
pub fn from_10p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 10%;")).unwrap()
}

/// Tailwind `from-15%`; css `--tw-gradient-from-position: 15%;`
pub fn from_15p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 15%;")).unwrap()
}

/// Tailwind `from-20%`; css `--tw-gradient-from-position: 20%;`
pub fn from_20p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 20%;")).unwrap()
}

/// Tailwind `from-25%`; css `--tw-gradient-from-position: 25%;`
pub fn from_25p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 25%;")).unwrap()
}

/// Tailwind `from-30%`; css `--tw-gradient-from-position: 30%;`
pub fn from_30p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 30%;")).unwrap()
}

/// Tailwind `from-35%`; css `--tw-gradient-from-position: 35%;`
pub fn from_35p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 35%;")).unwrap()
}

/// Tailwind `from-40%`; css `--tw-gradient-from-position: 40%;`
pub fn from_40p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 40%;")).unwrap()
}

/// Tailwind `from-45%`; css `--tw-gradient-from-position: 45%;`
pub fn from_45p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 45%;")).unwrap()
}

/// Tailwind `from-50%`; css `--tw-gradient-from-position: 50%;`
pub fn from_50p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 50%;")).unwrap()
}

/// Tailwind `from-55%`; css `--tw-gradient-from-position: 55%;`
pub fn from_55p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 55%;")).unwrap()
}

/// Tailwind `from-60%`; css `--tw-gradient-from-position: 60%;`
pub fn from_60p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 60%;")).unwrap()
}

/// Tailwind `from-65%`; css `--tw-gradient-from-position: 65%;`
pub fn from_65p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 65%;")).unwrap()
}

/// Tailwind `from-70%`; css `--tw-gradient-from-position: 70%;`
pub fn from_70p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 70%;")).unwrap()
}

/// Tailwind `from-75%`; css `--tw-gradient-from-position: 75%;`
pub fn from_75p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 75%;")).unwrap()
}

/// Tailwind `from-80%`; css `--tw-gradient-from-position: 80%;`
pub fn from_80p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 80%;")).unwrap()
}

/// Tailwind `from-85%`; css `--tw-gradient-from-position: 85%;`
pub fn from_85p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 85%;")).unwrap()
}

/// Tailwind `from-90%`; css `--tw-gradient-from-position: 90%;`
pub fn from_90p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 90%;")).unwrap()
}

/// Tailwind `from-95%`; css `--tw-gradient-from-position: 95%;`
pub fn from_95p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 95%;")).unwrap()
}

/// Tailwind `from-100%`; css `--tw-gradient-from-position: 100%;`
pub fn from_100p() -> Style {
    Style::new(css!("--tw-gradient-from-position: 100%;")).unwrap()
}

/// Tailwind `via-inherit`; css `--tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), inherit var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_inherit() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), inherit var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-current`; css `--tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), currentColor var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_current() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), currentColor var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-transparent`; css `--tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), transparent var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_transparent() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), transparent var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-black`; css `--tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #000 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_black() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #000 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-white`; css `--tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_white() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-50`; css `--tw-gradient-to: rgb(248 250 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f8fafc var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(248 250 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f8fafc var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-100`; css `--tw-gradient-to: rgb(241 245 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f1f5f9 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(241 245 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f1f5f9 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-200`; css `--tw-gradient-to: rgb(226 232 240 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e2e8f0 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(226 232 240 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e2e8f0 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-300`; css `--tw-gradient-to: rgb(203 213 225 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #cbd5e1 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(203 213 225 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #cbd5e1 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-400`; css `--tw-gradient-to: rgb(148 163 184 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #94a3b8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(148 163 184 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #94a3b8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-500`; css `--tw-gradient-to: rgb(100 116 139 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #64748b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(100 116 139 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #64748b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-600`; css `--tw-gradient-to: rgb(71 85 105 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #475569 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(71 85 105 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #475569 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-700`; css `--tw-gradient-to: rgb(51 65 85 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #334155 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(51 65 85 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #334155 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-800`; css `--tw-gradient-to: rgb(30 41 59 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1e293b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(30 41 59 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1e293b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-900`; css `--tw-gradient-to: rgb(15 23 42 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0f172a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(15 23 42 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0f172a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-slate-950`; css `--tw-gradient-to: rgb(2 6 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #020617 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_slate_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(2 6 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #020617 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-50`; css `--tw-gradient-to: rgb(249 250 251 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f9fafb var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(249 250 251 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f9fafb var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-100`; css `--tw-gradient-to: rgb(243 244 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f3f4f6 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(243 244 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f3f4f6 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-200`; css `--tw-gradient-to: rgb(229 231 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e5e7eb var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(229 231 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e5e7eb var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-300`; css `--tw-gradient-to: rgb(209 213 219 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d1d5db var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(209 213 219 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d1d5db var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-400`; css `--tw-gradient-to: rgb(156 163 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9ca3af var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(156 163 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9ca3af var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-500`; css `--tw-gradient-to: rgb(107 114 128 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6b7280 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(107 114 128 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6b7280 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-600`; css `--tw-gradient-to: rgb(75 85 99 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4b5563 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(75 85 99 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4b5563 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-700`; css `--tw-gradient-to: rgb(55 65 81 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #374151 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(55 65 81 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #374151 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-800`; css `--tw-gradient-to: rgb(31 41 55 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1f2937 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(31 41 55 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1f2937 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-900`; css `--tw-gradient-to: rgb(17 24 39 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #111827 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(17 24 39 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #111827 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-gray-950`; css `--tw-gradient-to: rgb(3 7 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #030712 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_gray_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(3 7 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #030712 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-50`; css `--tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fafafa var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fafafa var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-100`; css `--tw-gradient-to: rgb(244 244 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f4f4f5 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(244 244 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f4f4f5 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-200`; css `--tw-gradient-to: rgb(228 228 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e4e4e7 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(228 228 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e4e4e7 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-300`; css `--tw-gradient-to: rgb(212 212 216 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d4d4d8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(212 212 216 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d4d4d8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-400`; css `--tw-gradient-to: rgb(161 161 170 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a1a1aa var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(161 161 170 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a1a1aa var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-500`; css `--tw-gradient-to: rgb(113 113 122 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #71717a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(113 113 122 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #71717a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-600`; css `--tw-gradient-to: rgb(82 82 91 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #52525b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(82 82 91 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #52525b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-700`; css `--tw-gradient-to: rgb(63 63 70 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3f3f46 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(63 63 70 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3f3f46 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-800`; css `--tw-gradient-to: rgb(39 39 42 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #27272a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(39 39 42 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #27272a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-900`; css `--tw-gradient-to: rgb(24 24 27 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #18181b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(24 24 27 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #18181b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-zinc-950`; css `--tw-gradient-to: rgb(9 9 11 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #09090b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_zinc_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(9 9 11 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #09090b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-50`; css `--tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fafafa var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fafafa var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-100`; css `--tw-gradient-to: rgb(245 245 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f5f5f5 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(245 245 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f5f5f5 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-200`; css `--tw-gradient-to: rgb(229 229 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e5e5e5 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(229 229 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e5e5e5 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-300`; css `--tw-gradient-to: rgb(212 212 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d4d4d4 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(212 212 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d4d4d4 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-400`; css `--tw-gradient-to: rgb(163 163 163 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a3a3a3 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(163 163 163 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a3a3a3 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-500`; css `--tw-gradient-to: rgb(115 115 115 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #737373 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(115 115 115 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #737373 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-600`; css `--tw-gradient-to: rgb(82 82 82 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #525252 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(82 82 82 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #525252 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-700`; css `--tw-gradient-to: rgb(64 64 64 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #404040 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(64 64 64 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #404040 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-800`; css `--tw-gradient-to: rgb(38 38 38 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #262626 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(38 38 38 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #262626 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-900`; css `--tw-gradient-to: rgb(23 23 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #171717 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(23 23 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #171717 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-neutral-950`; css `--tw-gradient-to: rgb(10 10 10 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0a0a0a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_neutral_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(10 10 10 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0a0a0a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-50`; css `--tw-gradient-to: rgb(250 250 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fafaf9 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(250 250 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fafaf9 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-100`; css `--tw-gradient-to: rgb(245 245 244 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f5f5f4 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(245 245 244 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f5f5f4 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-200`; css `--tw-gradient-to: rgb(231 229 228 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e7e5e4 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(231 229 228 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e7e5e4 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-300`; css `--tw-gradient-to: rgb(214 211 209 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d6d3d1 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(214 211 209 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d6d3d1 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-400`; css `--tw-gradient-to: rgb(168 162 158 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a8a29e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(168 162 158 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a8a29e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-500`; css `--tw-gradient-to: rgb(120 113 108 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #78716c var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(120 113 108 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #78716c var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-600`; css `--tw-gradient-to: rgb(87 83 78 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #57534e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(87 83 78 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #57534e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-700`; css `--tw-gradient-to: rgb(68 64 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #44403c var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(68 64 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #44403c var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-800`; css `--tw-gradient-to: rgb(41 37 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #292524 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(41 37 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #292524 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-900`; css `--tw-gradient-to: rgb(28 25 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1c1917 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(28 25 23 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1c1917 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-stone-950`; css `--tw-gradient-to: rgb(12 10 9 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0c0a09 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_stone_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(12 10 9 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0c0a09 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-50`; css `--tw-gradient-to: rgb(254 242 242 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fef2f2 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 242 242 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fef2f2 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-100`; css `--tw-gradient-to: rgb(254 226 226 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fee2e2 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 226 226 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fee2e2 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-200`; css `--tw-gradient-to: rgb(254 202 202 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fecaca var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 202 202 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fecaca var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-300`; css `--tw-gradient-to: rgb(252 165 165 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fca5a5 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(252 165 165 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fca5a5 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-400`; css `--tw-gradient-to: rgb(248 113 113 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f87171 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(248 113 113 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f87171 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-500`; css `--tw-gradient-to: rgb(239 68 68 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ef4444 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(239 68 68 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ef4444 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-600`; css `--tw-gradient-to: rgb(220 38 38 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #dc2626 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(220 38 38 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #dc2626 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-700`; css `--tw-gradient-to: rgb(185 28 28 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #b91c1c var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(185 28 28 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #b91c1c var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-800`; css `--tw-gradient-to: rgb(153 27 27 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #991b1b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(153 27 27 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #991b1b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-900`; css `--tw-gradient-to: rgb(127 29 29 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7f1d1d var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(127 29 29 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7f1d1d var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-red-950`; css `--tw-gradient-to: rgb(69 10 10 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #450a0a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_red_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(69 10 10 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #450a0a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-50`; css `--tw-gradient-to: rgb(255 247 237 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fff7ed var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(255 247 237 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fff7ed var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-100`; css `--tw-gradient-to: rgb(255 237 213 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ffedd5 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(255 237 213 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ffedd5 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-200`; css `--tw-gradient-to: rgb(254 215 170 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fed7aa var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 215 170 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fed7aa var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-300`; css `--tw-gradient-to: rgb(253 186 116 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fdba74 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(253 186 116 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fdba74 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-400`; css `--tw-gradient-to: rgb(251 146 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fb923c var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(251 146 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fb923c var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-500`; css `--tw-gradient-to: rgb(249 115 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f97316 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(249 115 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f97316 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-600`; css `--tw-gradient-to: rgb(234 88 12 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ea580c var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(234 88 12 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ea580c var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-700`; css `--tw-gradient-to: rgb(194 65 12 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c2410c var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(194 65 12 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c2410c var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-800`; css `--tw-gradient-to: rgb(154 52 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9a3412 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(154 52 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9a3412 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-900`; css `--tw-gradient-to: rgb(124 45 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7c2d12 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(124 45 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7c2d12 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-orange-950`; css `--tw-gradient-to: rgb(67 20 7 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #431407 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_orange_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(67 20 7 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #431407 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-50`; css `--tw-gradient-to: rgb(255 251 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fffbeb var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(255 251 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fffbeb var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-100`; css `--tw-gradient-to: rgb(254 243 199 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fef3c7 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 243 199 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fef3c7 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-200`; css `--tw-gradient-to: rgb(253 230 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fde68a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(253 230 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fde68a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-300`; css `--tw-gradient-to: rgb(252 211 77 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fcd34d var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(252 211 77 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fcd34d var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-400`; css `--tw-gradient-to: rgb(251 191 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fbbf24 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(251 191 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fbbf24 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-500`; css `--tw-gradient-to: rgb(245 158 11 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f59e0b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(245 158 11 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f59e0b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-600`; css `--tw-gradient-to: rgb(217 119 6 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d97706 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(217 119 6 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d97706 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-700`; css `--tw-gradient-to: rgb(180 83 9 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #b45309 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(180 83 9 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #b45309 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-800`; css `--tw-gradient-to: rgb(146 64 14 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #92400e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(146 64 14 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #92400e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-900`; css `--tw-gradient-to: rgb(120 53 15 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #78350f var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(120 53 15 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #78350f var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-amber-950`; css `--tw-gradient-to: rgb(69 26 3 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #451a03 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_amber_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(69 26 3 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #451a03 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-50`; css `--tw-gradient-to: rgb(254 252 232 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fefce8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 252 232 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fefce8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-100`; css `--tw-gradient-to: rgb(254 249 195 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fef9c3 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 249 195 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fef9c3 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-200`; css `--tw-gradient-to: rgb(254 240 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fef08a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 240 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fef08a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-300`; css `--tw-gradient-to: rgb(253 224 71 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fde047 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(253 224 71 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fde047 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-400`; css `--tw-gradient-to: rgb(250 204 21 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #facc15 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(250 204 21 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #facc15 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-500`; css `--tw-gradient-to: rgb(234 179 8 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #eab308 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(234 179 8 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #eab308 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-600`; css `--tw-gradient-to: rgb(202 138 4 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ca8a04 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(202 138 4 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ca8a04 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-700`; css `--tw-gradient-to: rgb(161 98 7 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a16207 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(161 98 7 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a16207 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-800`; css `--tw-gradient-to: rgb(133 77 14 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #854d0e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(133 77 14 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #854d0e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-900`; css `--tw-gradient-to: rgb(113 63 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #713f12 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(113 63 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #713f12 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-yellow-950`; css `--tw-gradient-to: rgb(66 32 6 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #422006 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_yellow_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(66 32 6 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #422006 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-50`; css `--tw-gradient-to: rgb(247 254 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f7fee7 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(247 254 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f7fee7 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-100`; css `--tw-gradient-to: rgb(236 252 203 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ecfccb var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(236 252 203 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ecfccb var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-200`; css `--tw-gradient-to: rgb(217 249 157 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d9f99d var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(217 249 157 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d9f99d var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-300`; css `--tw-gradient-to: rgb(190 242 100 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #bef264 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(190 242 100 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #bef264 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-400`; css `--tw-gradient-to: rgb(163 230 53 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a3e635 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(163 230 53 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a3e635 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-500`; css `--tw-gradient-to: rgb(132 204 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #84cc16 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(132 204 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #84cc16 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-600`; css `--tw-gradient-to: rgb(101 163 13 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #65a30d var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(101 163 13 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #65a30d var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-700`; css `--tw-gradient-to: rgb(77 124 15 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4d7c0f var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(77 124 15 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4d7c0f var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-800`; css `--tw-gradient-to: rgb(63 98 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3f6212 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(63 98 18 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3f6212 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-900`; css `--tw-gradient-to: rgb(54 83 20 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #365314 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(54 83 20 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #365314 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-lime-950`; css `--tw-gradient-to: rgb(26 46 5 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1a2e05 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_lime_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(26 46 5 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1a2e05 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-50`; css `--tw-gradient-to: rgb(240 253 244 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f0fdf4 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(240 253 244 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f0fdf4 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-100`; css `--tw-gradient-to: rgb(220 252 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #dcfce7 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(220 252 231 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #dcfce7 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-200`; css `--tw-gradient-to: rgb(187 247 208 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #bbf7d0 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(187 247 208 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #bbf7d0 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-300`; css `--tw-gradient-to: rgb(134 239 172 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #86efac var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(134 239 172 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #86efac var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-400`; css `--tw-gradient-to: rgb(74 222 128 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4ade80 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(74 222 128 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4ade80 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-500`; css `--tw-gradient-to: rgb(34 197 94 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #22c55e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(34 197 94 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #22c55e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-600`; css `--tw-gradient-to: rgb(22 163 74 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #16a34a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(22 163 74 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #16a34a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-700`; css `--tw-gradient-to: rgb(21 128 61 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #15803d var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(21 128 61 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #15803d var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-800`; css `--tw-gradient-to: rgb(22 101 52 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #166534 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(22 101 52 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #166534 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-900`; css `--tw-gradient-to: rgb(20 83 45 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #14532d var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(20 83 45 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #14532d var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-green-950`; css `--tw-gradient-to: rgb(5 46 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #052e16 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_green_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(5 46 22 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #052e16 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-50`; css `--tw-gradient-to: rgb(236 253 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ecfdf5 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(236 253 245 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ecfdf5 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-100`; css `--tw-gradient-to: rgb(209 250 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d1fae5 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(209 250 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d1fae5 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-200`; css `--tw-gradient-to: rgb(167 243 208 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a7f3d0 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(167 243 208 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a7f3d0 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-300`; css `--tw-gradient-to: rgb(110 231 183 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6ee7b7 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(110 231 183 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6ee7b7 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-400`; css `--tw-gradient-to: rgb(52 211 153 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #34d399 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(52 211 153 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #34d399 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-500`; css `--tw-gradient-to: rgb(16 185 129 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #10b981 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(16 185 129 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #10b981 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-600`; css `--tw-gradient-to: rgb(5 150 105 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #059669 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(5 150 105 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #059669 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-700`; css `--tw-gradient-to: rgb(4 120 87 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #047857 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(4 120 87 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #047857 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-800`; css `--tw-gradient-to: rgb(6 95 70 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #065f46 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(6 95 70 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #065f46 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-900`; css `--tw-gradient-to: rgb(6 78 59 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #064e3b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(6 78 59 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #064e3b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-emerald-950`; css `--tw-gradient-to: rgb(2 44 34 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #022c22 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_emerald_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(2 44 34 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #022c22 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-50`; css `--tw-gradient-to: rgb(240 253 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f0fdfa var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(240 253 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f0fdfa var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-100`; css `--tw-gradient-to: rgb(204 251 241 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ccfbf1 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(204 251 241 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ccfbf1 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-200`; css `--tw-gradient-to: rgb(153 246 228 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #99f6e4 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(153 246 228 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #99f6e4 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-300`; css `--tw-gradient-to: rgb(94 234 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #5eead4 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(94 234 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #5eead4 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-400`; css `--tw-gradient-to: rgb(45 212 191 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #2dd4bf var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(45 212 191 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #2dd4bf var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-500`; css `--tw-gradient-to: rgb(20 184 166 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #14b8a6 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(20 184 166 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #14b8a6 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-600`; css `--tw-gradient-to: rgb(13 148 136 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0d9488 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(13 148 136 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0d9488 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-700`; css `--tw-gradient-to: rgb(15 118 110 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0f766e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(15 118 110 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0f766e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-800`; css `--tw-gradient-to: rgb(17 94 89 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #115e59 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(17 94 89 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #115e59 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-900`; css `--tw-gradient-to: rgb(19 78 74 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #134e4a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(19 78 74 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #134e4a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-teal-950`; css `--tw-gradient-to: rgb(4 47 46 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #042f2e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_teal_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(4 47 46 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #042f2e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-50`; css `--tw-gradient-to: rgb(236 254 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ecfeff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(236 254 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ecfeff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-100`; css `--tw-gradient-to: rgb(207 250 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #cffafe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(207 250 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #cffafe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-200`; css `--tw-gradient-to: rgb(165 243 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a5f3fc var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(165 243 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a5f3fc var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-300`; css `--tw-gradient-to: rgb(103 232 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #67e8f9 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(103 232 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #67e8f9 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-400`; css `--tw-gradient-to: rgb(34 211 238 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #22d3ee var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(34 211 238 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #22d3ee var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-500`; css `--tw-gradient-to: rgb(6 182 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #06b6d4 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(6 182 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #06b6d4 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-600`; css `--tw-gradient-to: rgb(8 145 178 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0891b2 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(8 145 178 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0891b2 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-700`; css `--tw-gradient-to: rgb(14 116 144 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0e7490 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(14 116 144 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0e7490 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-800`; css `--tw-gradient-to: rgb(21 94 117 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #155e75 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(21 94 117 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #155e75 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-900`; css `--tw-gradient-to: rgb(22 78 99 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #164e63 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(22 78 99 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #164e63 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-cyan-950`; css `--tw-gradient-to: rgb(8 51 68 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #083344 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_cyan_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(8 51 68 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #083344 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-50`; css `--tw-gradient-to: rgb(240 249 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f0f9ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(240 249 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f0f9ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-100`; css `--tw-gradient-to: rgb(224 242 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e0f2fe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(224 242 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e0f2fe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-200`; css `--tw-gradient-to: rgb(186 230 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #bae6fd var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(186 230 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #bae6fd var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-300`; css `--tw-gradient-to: rgb(125 211 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7dd3fc var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(125 211 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7dd3fc var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-400`; css `--tw-gradient-to: rgb(56 189 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #38bdf8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(56 189 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #38bdf8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-500`; css `--tw-gradient-to: rgb(14 165 233 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0ea5e9 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(14 165 233 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0ea5e9 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-600`; css `--tw-gradient-to: rgb(2 132 199 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0284c7 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(2 132 199 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0284c7 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-700`; css `--tw-gradient-to: rgb(3 105 161 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0369a1 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(3 105 161 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0369a1 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-800`; css `--tw-gradient-to: rgb(7 89 133 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #075985 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(7 89 133 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #075985 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-900`; css `--tw-gradient-to: rgb(12 74 110 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0c4a6e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(12 74 110 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #0c4a6e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-sky-950`; css `--tw-gradient-to: rgb(8 47 73 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #082f49 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_sky_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(8 47 73 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #082f49 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-50`; css `--tw-gradient-to: rgb(239 246 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #eff6ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(239 246 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #eff6ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-100`; css `--tw-gradient-to: rgb(219 234 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #dbeafe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(219 234 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #dbeafe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-200`; css `--tw-gradient-to: rgb(191 219 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #bfdbfe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(191 219 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #bfdbfe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-300`; css `--tw-gradient-to: rgb(147 197 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #93c5fd var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(147 197 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #93c5fd var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-400`; css `--tw-gradient-to: rgb(96 165 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #60a5fa var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(96 165 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #60a5fa var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-500`; css `--tw-gradient-to: rgb(59 130 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3b82f6 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(59 130 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3b82f6 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-600`; css `--tw-gradient-to: rgb(37 99 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #2563eb var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(37 99 235 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #2563eb var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-700`; css `--tw-gradient-to: rgb(29 78 216 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1d4ed8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(29 78 216 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1d4ed8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-800`; css `--tw-gradient-to: rgb(30 64 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1e40af var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(30 64 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1e40af var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-900`; css `--tw-gradient-to: rgb(30 58 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1e3a8a var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(30 58 138 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1e3a8a var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-blue-950`; css `--tw-gradient-to: rgb(23 37 84 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #172554 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_blue_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(23 37 84 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #172554 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-50`; css `--tw-gradient-to: rgb(238 242 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #eef2ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(238 242 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #eef2ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-100`; css `--tw-gradient-to: rgb(224 231 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e0e7ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(224 231 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e0e7ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-200`; css `--tw-gradient-to: rgb(199 210 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c7d2fe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(199 210 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c7d2fe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-300`; css `--tw-gradient-to: rgb(165 180 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a5b4fc var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(165 180 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a5b4fc var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-400`; css `--tw-gradient-to: rgb(129 140 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #818cf8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(129 140 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #818cf8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-500`; css `--tw-gradient-to: rgb(99 102 241 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6366f1 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(99 102 241 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6366f1 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-600`; css `--tw-gradient-to: rgb(79 70 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4f46e5 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(79 70 229 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4f46e5 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-700`; css `--tw-gradient-to: rgb(67 56 202 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4338ca var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(67 56 202 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4338ca var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-800`; css `--tw-gradient-to: rgb(55 48 163 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3730a3 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(55 48 163 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3730a3 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-900`; css `--tw-gradient-to: rgb(49 46 129 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #312e81 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(49 46 129 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #312e81 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-indigo-950`; css `--tw-gradient-to: rgb(30 27 75 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1e1b4b var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_indigo_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(30 27 75 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #1e1b4b var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-50`; css `--tw-gradient-to: rgb(245 243 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f5f3ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(245 243 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f5f3ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-100`; css `--tw-gradient-to: rgb(237 233 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ede9fe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(237 233 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ede9fe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-200`; css `--tw-gradient-to: rgb(221 214 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ddd6fe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(221 214 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ddd6fe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-300`; css `--tw-gradient-to: rgb(196 181 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c4b5fd var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(196 181 253 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c4b5fd var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-400`; css `--tw-gradient-to: rgb(167 139 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a78bfa var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(167 139 250 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a78bfa var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-500`; css `--tw-gradient-to: rgb(139 92 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #8b5cf6 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(139 92 246 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #8b5cf6 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-600`; css `--tw-gradient-to: rgb(124 58 237 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7c3aed var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(124 58 237 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7c3aed var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-700`; css `--tw-gradient-to: rgb(109 40 217 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6d28d9 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(109 40 217 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6d28d9 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-800`; css `--tw-gradient-to: rgb(91 33 182 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #5b21b6 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(91 33 182 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #5b21b6 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-900`; css `--tw-gradient-to: rgb(76 29 149 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4c1d95 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(76 29 149 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4c1d95 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-violet-950`; css `--tw-gradient-to: rgb(46 16 101 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #2e1065 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_violet_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(46 16 101 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #2e1065 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-50`; css `--tw-gradient-to: rgb(250 245 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #faf5ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(250 245 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #faf5ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-100`; css `--tw-gradient-to: rgb(243 232 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f3e8ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(243 232 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f3e8ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-200`; css `--tw-gradient-to: rgb(233 213 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e9d5ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(233 213 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e9d5ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-300`; css `--tw-gradient-to: rgb(216 180 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d8b4fe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(216 180 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d8b4fe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-400`; css `--tw-gradient-to: rgb(192 132 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c084fc var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(192 132 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c084fc var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-500`; css `--tw-gradient-to: rgb(168 85 247 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a855f7 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(168 85 247 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a855f7 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-600`; css `--tw-gradient-to: rgb(147 51 234 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9333ea var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(147 51 234 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9333ea var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-700`; css `--tw-gradient-to: rgb(126 34 206 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7e22ce var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(126 34 206 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #7e22ce var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-800`; css `--tw-gradient-to: rgb(107 33 168 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6b21a8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(107 33 168 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #6b21a8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-900`; css `--tw-gradient-to: rgb(88 28 135 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #581c87 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(88 28 135 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #581c87 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-purple-950`; css `--tw-gradient-to: rgb(59 7 100 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3b0764 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_purple_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(59 7 100 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #3b0764 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-50`; css `--tw-gradient-to: rgb(253 244 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fdf4ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(253 244 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fdf4ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-100`; css `--tw-gradient-to: rgb(250 232 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fae8ff var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(250 232 255 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fae8ff var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-200`; css `--tw-gradient-to: rgb(245 208 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f5d0fe var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(245 208 254 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f5d0fe var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-300`; css `--tw-gradient-to: rgb(240 171 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f0abfc var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(240 171 252 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f0abfc var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-400`; css `--tw-gradient-to: rgb(232 121 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e879f9 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(232 121 249 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e879f9 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-500`; css `--tw-gradient-to: rgb(217 70 239 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d946ef var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(217 70 239 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #d946ef var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-600`; css `--tw-gradient-to: rgb(192 38 211 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c026d3 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(192 38 211 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #c026d3 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-700`; css `--tw-gradient-to: rgb(162 28 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a21caf var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(162 28 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #a21caf var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-800`; css `--tw-gradient-to: rgb(134 25 143 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #86198f var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(134 25 143 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #86198f var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-900`; css `--tw-gradient-to: rgb(112 26 117 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #701a75 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(112 26 117 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #701a75 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-fuchsia-950`; css `--tw-gradient-to: rgb(74 4 78 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4a044e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_fuchsia_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(74 4 78 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4a044e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-50`; css `--tw-gradient-to: rgb(253 242 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fdf2f8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(253 242 248 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fdf2f8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-100`; css `--tw-gradient-to: rgb(252 231 243 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fce7f3 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(252 231 243 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fce7f3 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-200`; css `--tw-gradient-to: rgb(251 207 232 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fbcfe8 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(251 207 232 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fbcfe8 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-300`; css `--tw-gradient-to: rgb(249 168 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f9a8d4 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(249 168 212 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f9a8d4 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-400`; css `--tw-gradient-to: rgb(244 114 182 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f472b6 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(244 114 182 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f472b6 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-500`; css `--tw-gradient-to: rgb(236 72 153 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ec4899 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(236 72 153 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ec4899 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-600`; css `--tw-gradient-to: rgb(219 39 119 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #db2777 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(219 39 119 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #db2777 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-700`; css `--tw-gradient-to: rgb(190 24 93 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #be185d var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(190 24 93 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #be185d var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-800`; css `--tw-gradient-to: rgb(157 23 77 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9d174d var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(157 23 77 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9d174d var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-900`; css `--tw-gradient-to: rgb(131 24 67 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #831843 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(131 24 67 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #831843 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-pink-950`; css `--tw-gradient-to: rgb(80 7 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #500724 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_pink_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(80 7 36 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #500724 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-50`; css `--tw-gradient-to: rgb(255 241 242 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fff1f2 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_50() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(255 241 242 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fff1f2 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-100`; css `--tw-gradient-to: rgb(255 228 230 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ffe4e6 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_100() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(255 228 230 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #ffe4e6 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-200`; css `--tw-gradient-to: rgb(254 205 211 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fecdd3 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_200() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(254 205 211 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fecdd3 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-300`; css `--tw-gradient-to: rgb(253 164 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fda4af var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_300() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(253 164 175 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fda4af var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-400`; css `--tw-gradient-to: rgb(251 113 133 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fb7185 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_400() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(251 113 133 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #fb7185 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-500`; css `--tw-gradient-to: rgb(244 63 94 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f43f5e var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_500() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(244 63 94 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #f43f5e var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-600`; css `--tw-gradient-to: rgb(225 29 72 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e11d48 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_600() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(225 29 72 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #e11d48 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-700`; css `--tw-gradient-to: rgb(190 18 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #be123c var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_700() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(190 18 60 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #be123c var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-800`; css `--tw-gradient-to: rgb(159 18 57 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9f1239 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_800() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(159 18 57 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #9f1239 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-900`; css `--tw-gradient-to: rgb(136 19 55 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #881337 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_900() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(136 19 55 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #881337 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-rose-950`; css `--tw-gradient-to: rgb(76 5 25 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4c0519 var(--tw-gradient-via-position), var(--tw-gradient-to);`
pub fn via_rose_950() -> Style {
    Style::new(css!("--tw-gradient-to: rgb(76 5 25 / 0) var(--tw-gradient-to-position); --tw-gradient-stops: var(--tw-gradient-from), #4c0519 var(--tw-gradient-via-position), var(--tw-gradient-to);")).unwrap()
}

/// Tailwind `via-0%`; css `--tw-gradient-via-position: 0%;`
pub fn via_0p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 0%;")).unwrap()
}

/// Tailwind `via-5%`; css `--tw-gradient-via-position: 5%;`
pub fn via_5p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 5%;")).unwrap()
}

/// Tailwind `via-10%`; css `--tw-gradient-via-position: 10%;`
pub fn via_10p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 10%;")).unwrap()
}

/// Tailwind `via-15%`; css `--tw-gradient-via-position: 15%;`
pub fn via_15p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 15%;")).unwrap()
}

/// Tailwind `via-20%`; css `--tw-gradient-via-position: 20%;`
pub fn via_20p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 20%;")).unwrap()
}

/// Tailwind `via-25%`; css `--tw-gradient-via-position: 25%;`
pub fn via_25p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 25%;")).unwrap()
}

/// Tailwind `via-30%`; css `--tw-gradient-via-position: 30%;`
pub fn via_30p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 30%;")).unwrap()
}

/// Tailwind `via-35%`; css `--tw-gradient-via-position: 35%;`
pub fn via_35p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 35%;")).unwrap()
}

/// Tailwind `via-40%`; css `--tw-gradient-via-position: 40%;`
pub fn via_40p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 40%;")).unwrap()
}

/// Tailwind `via-45%`; css `--tw-gradient-via-position: 45%;`
pub fn via_45p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 45%;")).unwrap()
}

/// Tailwind `via-50%`; css `--tw-gradient-via-position: 50%;`
pub fn via_50p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 50%;")).unwrap()
}

/// Tailwind `via-55%`; css `--tw-gradient-via-position: 55%;`
pub fn via_55p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 55%;")).unwrap()
}

/// Tailwind `via-60%`; css `--tw-gradient-via-position: 60%;`
pub fn via_60p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 60%;")).unwrap()
}

/// Tailwind `via-65%`; css `--tw-gradient-via-position: 65%;`
pub fn via_65p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 65%;")).unwrap()
}

/// Tailwind `via-70%`; css `--tw-gradient-via-position: 70%;`
pub fn via_70p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 70%;")).unwrap()
}

/// Tailwind `via-75%`; css `--tw-gradient-via-position: 75%;`
pub fn via_75p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 75%;")).unwrap()
}

/// Tailwind `via-80%`; css `--tw-gradient-via-position: 80%;`
pub fn via_80p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 80%;")).unwrap()
}

/// Tailwind `via-85%`; css `--tw-gradient-via-position: 85%;`
pub fn via_85p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 85%;")).unwrap()
}

/// Tailwind `via-90%`; css `--tw-gradient-via-position: 90%;`
pub fn via_90p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 90%;")).unwrap()
}

/// Tailwind `via-95%`; css `--tw-gradient-via-position: 95%;`
pub fn via_95p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 95%;")).unwrap()
}

/// Tailwind `via-100%`; css `--tw-gradient-via-position: 100%;`
pub fn via_100p() -> Style {
    Style::new(css!("--tw-gradient-via-position: 100%;")).unwrap()
}

/// Tailwind `to-inherit`; css `--tw-gradient-to: inherit var(--tw-gradient-to-position);`
pub fn to_inherit() -> Style {
    Style::new(css!("--tw-gradient-to: inherit var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-current`; css `--tw-gradient-to: currentColor var(--tw-gradient-to-position);`
pub fn to_current() -> Style {
    Style::new(css!("--tw-gradient-to: currentColor var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-transparent`; css `--tw-gradient-to: transparent var(--tw-gradient-to-position);`
pub fn to_transparent() -> Style {
    Style::new(css!("--tw-gradient-to: transparent var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-black`; css `--tw-gradient-to: #000 var(--tw-gradient-to-position);`
pub fn to_black() -> Style {
    Style::new(css!("--tw-gradient-to: #000 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-white`; css `--tw-gradient-to: #fff var(--tw-gradient-to-position);`
pub fn to_white() -> Style {
    Style::new(css!("--tw-gradient-to: #fff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-50`; css `--tw-gradient-to: #f8fafc var(--tw-gradient-to-position);`
pub fn to_slate_50() -> Style {
    Style::new(css!("--tw-gradient-to: #f8fafc var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-100`; css `--tw-gradient-to: #f1f5f9 var(--tw-gradient-to-position);`
pub fn to_slate_100() -> Style {
    Style::new(css!("--tw-gradient-to: #f1f5f9 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-200`; css `--tw-gradient-to: #e2e8f0 var(--tw-gradient-to-position);`
pub fn to_slate_200() -> Style {
    Style::new(css!("--tw-gradient-to: #e2e8f0 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-300`; css `--tw-gradient-to: #cbd5e1 var(--tw-gradient-to-position);`
pub fn to_slate_300() -> Style {
    Style::new(css!("--tw-gradient-to: #cbd5e1 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-400`; css `--tw-gradient-to: #94a3b8 var(--tw-gradient-to-position);`
pub fn to_slate_400() -> Style {
    Style::new(css!("--tw-gradient-to: #94a3b8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-500`; css `--tw-gradient-to: #64748b var(--tw-gradient-to-position);`
pub fn to_slate_500() -> Style {
    Style::new(css!("--tw-gradient-to: #64748b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-600`; css `--tw-gradient-to: #475569 var(--tw-gradient-to-position);`
pub fn to_slate_600() -> Style {
    Style::new(css!("--tw-gradient-to: #475569 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-700`; css `--tw-gradient-to: #334155 var(--tw-gradient-to-position);`
pub fn to_slate_700() -> Style {
    Style::new(css!("--tw-gradient-to: #334155 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-800`; css `--tw-gradient-to: #1e293b var(--tw-gradient-to-position);`
pub fn to_slate_800() -> Style {
    Style::new(css!("--tw-gradient-to: #1e293b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-900`; css `--tw-gradient-to: #0f172a var(--tw-gradient-to-position);`
pub fn to_slate_900() -> Style {
    Style::new(css!("--tw-gradient-to: #0f172a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-slate-950`; css `--tw-gradient-to: #020617 var(--tw-gradient-to-position);`
pub fn to_slate_950() -> Style {
    Style::new(css!("--tw-gradient-to: #020617 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-50`; css `--tw-gradient-to: #f9fafb var(--tw-gradient-to-position);`
pub fn to_gray_50() -> Style {
    Style::new(css!("--tw-gradient-to: #f9fafb var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-100`; css `--tw-gradient-to: #f3f4f6 var(--tw-gradient-to-position);`
pub fn to_gray_100() -> Style {
    Style::new(css!("--tw-gradient-to: #f3f4f6 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-200`; css `--tw-gradient-to: #e5e7eb var(--tw-gradient-to-position);`
pub fn to_gray_200() -> Style {
    Style::new(css!("--tw-gradient-to: #e5e7eb var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-300`; css `--tw-gradient-to: #d1d5db var(--tw-gradient-to-position);`
pub fn to_gray_300() -> Style {
    Style::new(css!("--tw-gradient-to: #d1d5db var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-400`; css `--tw-gradient-to: #9ca3af var(--tw-gradient-to-position);`
pub fn to_gray_400() -> Style {
    Style::new(css!("--tw-gradient-to: #9ca3af var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-500`; css `--tw-gradient-to: #6b7280 var(--tw-gradient-to-position);`
pub fn to_gray_500() -> Style {
    Style::new(css!("--tw-gradient-to: #6b7280 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-600`; css `--tw-gradient-to: #4b5563 var(--tw-gradient-to-position);`
pub fn to_gray_600() -> Style {
    Style::new(css!("--tw-gradient-to: #4b5563 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-700`; css `--tw-gradient-to: #374151 var(--tw-gradient-to-position);`
pub fn to_gray_700() -> Style {
    Style::new(css!("--tw-gradient-to: #374151 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-800`; css `--tw-gradient-to: #1f2937 var(--tw-gradient-to-position);`
pub fn to_gray_800() -> Style {
    Style::new(css!("--tw-gradient-to: #1f2937 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-900`; css `--tw-gradient-to: #111827 var(--tw-gradient-to-position);`
pub fn to_gray_900() -> Style {
    Style::new(css!("--tw-gradient-to: #111827 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-gray-950`; css `--tw-gradient-to: #030712 var(--tw-gradient-to-position);`
pub fn to_gray_950() -> Style {
    Style::new(css!("--tw-gradient-to: #030712 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-50`; css `--tw-gradient-to: #fafafa var(--tw-gradient-to-position);`
pub fn to_zinc_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fafafa var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-100`; css `--tw-gradient-to: #f4f4f5 var(--tw-gradient-to-position);`
pub fn to_zinc_100() -> Style {
    Style::new(css!("--tw-gradient-to: #f4f4f5 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-200`; css `--tw-gradient-to: #e4e4e7 var(--tw-gradient-to-position);`
pub fn to_zinc_200() -> Style {
    Style::new(css!("--tw-gradient-to: #e4e4e7 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-300`; css `--tw-gradient-to: #d4d4d8 var(--tw-gradient-to-position);`
pub fn to_zinc_300() -> Style {
    Style::new(css!("--tw-gradient-to: #d4d4d8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-400`; css `--tw-gradient-to: #a1a1aa var(--tw-gradient-to-position);`
pub fn to_zinc_400() -> Style {
    Style::new(css!("--tw-gradient-to: #a1a1aa var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-500`; css `--tw-gradient-to: #71717a var(--tw-gradient-to-position);`
pub fn to_zinc_500() -> Style {
    Style::new(css!("--tw-gradient-to: #71717a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-600`; css `--tw-gradient-to: #52525b var(--tw-gradient-to-position);`
pub fn to_zinc_600() -> Style {
    Style::new(css!("--tw-gradient-to: #52525b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-700`; css `--tw-gradient-to: #3f3f46 var(--tw-gradient-to-position);`
pub fn to_zinc_700() -> Style {
    Style::new(css!("--tw-gradient-to: #3f3f46 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-800`; css `--tw-gradient-to: #27272a var(--tw-gradient-to-position);`
pub fn to_zinc_800() -> Style {
    Style::new(css!("--tw-gradient-to: #27272a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-900`; css `--tw-gradient-to: #18181b var(--tw-gradient-to-position);`
pub fn to_zinc_900() -> Style {
    Style::new(css!("--tw-gradient-to: #18181b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-zinc-950`; css `--tw-gradient-to: #09090b var(--tw-gradient-to-position);`
pub fn to_zinc_950() -> Style {
    Style::new(css!("--tw-gradient-to: #09090b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-50`; css `--tw-gradient-to: #fafafa var(--tw-gradient-to-position);`
pub fn to_neutral_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fafafa var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-100`; css `--tw-gradient-to: #f5f5f5 var(--tw-gradient-to-position);`
pub fn to_neutral_100() -> Style {
    Style::new(css!("--tw-gradient-to: #f5f5f5 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-200`; css `--tw-gradient-to: #e5e5e5 var(--tw-gradient-to-position);`
pub fn to_neutral_200() -> Style {
    Style::new(css!("--tw-gradient-to: #e5e5e5 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-300`; css `--tw-gradient-to: #d4d4d4 var(--tw-gradient-to-position);`
pub fn to_neutral_300() -> Style {
    Style::new(css!("--tw-gradient-to: #d4d4d4 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-400`; css `--tw-gradient-to: #a3a3a3 var(--tw-gradient-to-position);`
pub fn to_neutral_400() -> Style {
    Style::new(css!("--tw-gradient-to: #a3a3a3 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-500`; css `--tw-gradient-to: #737373 var(--tw-gradient-to-position);`
pub fn to_neutral_500() -> Style {
    Style::new(css!("--tw-gradient-to: #737373 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-600`; css `--tw-gradient-to: #525252 var(--tw-gradient-to-position);`
pub fn to_neutral_600() -> Style {
    Style::new(css!("--tw-gradient-to: #525252 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-700`; css `--tw-gradient-to: #404040 var(--tw-gradient-to-position);`
pub fn to_neutral_700() -> Style {
    Style::new(css!("--tw-gradient-to: #404040 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-800`; css `--tw-gradient-to: #262626 var(--tw-gradient-to-position);`
pub fn to_neutral_800() -> Style {
    Style::new(css!("--tw-gradient-to: #262626 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-900`; css `--tw-gradient-to: #171717 var(--tw-gradient-to-position);`
pub fn to_neutral_900() -> Style {
    Style::new(css!("--tw-gradient-to: #171717 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-neutral-950`; css `--tw-gradient-to: #0a0a0a var(--tw-gradient-to-position);`
pub fn to_neutral_950() -> Style {
    Style::new(css!("--tw-gradient-to: #0a0a0a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-50`; css `--tw-gradient-to: #fafaf9 var(--tw-gradient-to-position);`
pub fn to_stone_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fafaf9 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-100`; css `--tw-gradient-to: #f5f5f4 var(--tw-gradient-to-position);`
pub fn to_stone_100() -> Style {
    Style::new(css!("--tw-gradient-to: #f5f5f4 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-200`; css `--tw-gradient-to: #e7e5e4 var(--tw-gradient-to-position);`
pub fn to_stone_200() -> Style {
    Style::new(css!("--tw-gradient-to: #e7e5e4 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-300`; css `--tw-gradient-to: #d6d3d1 var(--tw-gradient-to-position);`
pub fn to_stone_300() -> Style {
    Style::new(css!("--tw-gradient-to: #d6d3d1 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-400`; css `--tw-gradient-to: #a8a29e var(--tw-gradient-to-position);`
pub fn to_stone_400() -> Style {
    Style::new(css!("--tw-gradient-to: #a8a29e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-500`; css `--tw-gradient-to: #78716c var(--tw-gradient-to-position);`
pub fn to_stone_500() -> Style {
    Style::new(css!("--tw-gradient-to: #78716c var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-600`; css `--tw-gradient-to: #57534e var(--tw-gradient-to-position);`
pub fn to_stone_600() -> Style {
    Style::new(css!("--tw-gradient-to: #57534e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-700`; css `--tw-gradient-to: #44403c var(--tw-gradient-to-position);`
pub fn to_stone_700() -> Style {
    Style::new(css!("--tw-gradient-to: #44403c var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-800`; css `--tw-gradient-to: #292524 var(--tw-gradient-to-position);`
pub fn to_stone_800() -> Style {
    Style::new(css!("--tw-gradient-to: #292524 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-900`; css `--tw-gradient-to: #1c1917 var(--tw-gradient-to-position);`
pub fn to_stone_900() -> Style {
    Style::new(css!("--tw-gradient-to: #1c1917 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-stone-950`; css `--tw-gradient-to: #0c0a09 var(--tw-gradient-to-position);`
pub fn to_stone_950() -> Style {
    Style::new(css!("--tw-gradient-to: #0c0a09 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-50`; css `--tw-gradient-to: #fef2f2 var(--tw-gradient-to-position);`
pub fn to_red_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fef2f2 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-100`; css `--tw-gradient-to: #fee2e2 var(--tw-gradient-to-position);`
pub fn to_red_100() -> Style {
    Style::new(css!("--tw-gradient-to: #fee2e2 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-200`; css `--tw-gradient-to: #fecaca var(--tw-gradient-to-position);`
pub fn to_red_200() -> Style {
    Style::new(css!("--tw-gradient-to: #fecaca var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-300`; css `--tw-gradient-to: #fca5a5 var(--tw-gradient-to-position);`
pub fn to_red_300() -> Style {
    Style::new(css!("--tw-gradient-to: #fca5a5 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-400`; css `--tw-gradient-to: #f87171 var(--tw-gradient-to-position);`
pub fn to_red_400() -> Style {
    Style::new(css!("--tw-gradient-to: #f87171 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-500`; css `--tw-gradient-to: #ef4444 var(--tw-gradient-to-position);`
pub fn to_red_500() -> Style {
    Style::new(css!("--tw-gradient-to: #ef4444 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-600`; css `--tw-gradient-to: #dc2626 var(--tw-gradient-to-position);`
pub fn to_red_600() -> Style {
    Style::new(css!("--tw-gradient-to: #dc2626 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-700`; css `--tw-gradient-to: #b91c1c var(--tw-gradient-to-position);`
pub fn to_red_700() -> Style {
    Style::new(css!("--tw-gradient-to: #b91c1c var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-800`; css `--tw-gradient-to: #991b1b var(--tw-gradient-to-position);`
pub fn to_red_800() -> Style {
    Style::new(css!("--tw-gradient-to: #991b1b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-900`; css `--tw-gradient-to: #7f1d1d var(--tw-gradient-to-position);`
pub fn to_red_900() -> Style {
    Style::new(css!("--tw-gradient-to: #7f1d1d var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-red-950`; css `--tw-gradient-to: #450a0a var(--tw-gradient-to-position);`
pub fn to_red_950() -> Style {
    Style::new(css!("--tw-gradient-to: #450a0a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-50`; css `--tw-gradient-to: #fff7ed var(--tw-gradient-to-position);`
pub fn to_orange_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fff7ed var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-100`; css `--tw-gradient-to: #ffedd5 var(--tw-gradient-to-position);`
pub fn to_orange_100() -> Style {
    Style::new(css!("--tw-gradient-to: #ffedd5 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-200`; css `--tw-gradient-to: #fed7aa var(--tw-gradient-to-position);`
pub fn to_orange_200() -> Style {
    Style::new(css!("--tw-gradient-to: #fed7aa var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-300`; css `--tw-gradient-to: #fdba74 var(--tw-gradient-to-position);`
pub fn to_orange_300() -> Style {
    Style::new(css!("--tw-gradient-to: #fdba74 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-400`; css `--tw-gradient-to: #fb923c var(--tw-gradient-to-position);`
pub fn to_orange_400() -> Style {
    Style::new(css!("--tw-gradient-to: #fb923c var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-500`; css `--tw-gradient-to: #f97316 var(--tw-gradient-to-position);`
pub fn to_orange_500() -> Style {
    Style::new(css!("--tw-gradient-to: #f97316 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-600`; css `--tw-gradient-to: #ea580c var(--tw-gradient-to-position);`
pub fn to_orange_600() -> Style {
    Style::new(css!("--tw-gradient-to: #ea580c var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-700`; css `--tw-gradient-to: #c2410c var(--tw-gradient-to-position);`
pub fn to_orange_700() -> Style {
    Style::new(css!("--tw-gradient-to: #c2410c var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-800`; css `--tw-gradient-to: #9a3412 var(--tw-gradient-to-position);`
pub fn to_orange_800() -> Style {
    Style::new(css!("--tw-gradient-to: #9a3412 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-900`; css `--tw-gradient-to: #7c2d12 var(--tw-gradient-to-position);`
pub fn to_orange_900() -> Style {
    Style::new(css!("--tw-gradient-to: #7c2d12 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-orange-950`; css `--tw-gradient-to: #431407 var(--tw-gradient-to-position);`
pub fn to_orange_950() -> Style {
    Style::new(css!("--tw-gradient-to: #431407 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-50`; css `--tw-gradient-to: #fffbeb var(--tw-gradient-to-position);`
pub fn to_amber_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fffbeb var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-100`; css `--tw-gradient-to: #fef3c7 var(--tw-gradient-to-position);`
pub fn to_amber_100() -> Style {
    Style::new(css!("--tw-gradient-to: #fef3c7 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-200`; css `--tw-gradient-to: #fde68a var(--tw-gradient-to-position);`
pub fn to_amber_200() -> Style {
    Style::new(css!("--tw-gradient-to: #fde68a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-300`; css `--tw-gradient-to: #fcd34d var(--tw-gradient-to-position);`
pub fn to_amber_300() -> Style {
    Style::new(css!("--tw-gradient-to: #fcd34d var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-400`; css `--tw-gradient-to: #fbbf24 var(--tw-gradient-to-position);`
pub fn to_amber_400() -> Style {
    Style::new(css!("--tw-gradient-to: #fbbf24 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-500`; css `--tw-gradient-to: #f59e0b var(--tw-gradient-to-position);`
pub fn to_amber_500() -> Style {
    Style::new(css!("--tw-gradient-to: #f59e0b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-600`; css `--tw-gradient-to: #d97706 var(--tw-gradient-to-position);`
pub fn to_amber_600() -> Style {
    Style::new(css!("--tw-gradient-to: #d97706 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-700`; css `--tw-gradient-to: #b45309 var(--tw-gradient-to-position);`
pub fn to_amber_700() -> Style {
    Style::new(css!("--tw-gradient-to: #b45309 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-800`; css `--tw-gradient-to: #92400e var(--tw-gradient-to-position);`
pub fn to_amber_800() -> Style {
    Style::new(css!("--tw-gradient-to: #92400e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-900`; css `--tw-gradient-to: #78350f var(--tw-gradient-to-position);`
pub fn to_amber_900() -> Style {
    Style::new(css!("--tw-gradient-to: #78350f var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-amber-950`; css `--tw-gradient-to: #451a03 var(--tw-gradient-to-position);`
pub fn to_amber_950() -> Style {
    Style::new(css!("--tw-gradient-to: #451a03 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-50`; css `--tw-gradient-to: #fefce8 var(--tw-gradient-to-position);`
pub fn to_yellow_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fefce8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-100`; css `--tw-gradient-to: #fef9c3 var(--tw-gradient-to-position);`
pub fn to_yellow_100() -> Style {
    Style::new(css!("--tw-gradient-to: #fef9c3 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-200`; css `--tw-gradient-to: #fef08a var(--tw-gradient-to-position);`
pub fn to_yellow_200() -> Style {
    Style::new(css!("--tw-gradient-to: #fef08a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-300`; css `--tw-gradient-to: #fde047 var(--tw-gradient-to-position);`
pub fn to_yellow_300() -> Style {
    Style::new(css!("--tw-gradient-to: #fde047 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-400`; css `--tw-gradient-to: #facc15 var(--tw-gradient-to-position);`
pub fn to_yellow_400() -> Style {
    Style::new(css!("--tw-gradient-to: #facc15 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-500`; css `--tw-gradient-to: #eab308 var(--tw-gradient-to-position);`
pub fn to_yellow_500() -> Style {
    Style::new(css!("--tw-gradient-to: #eab308 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-600`; css `--tw-gradient-to: #ca8a04 var(--tw-gradient-to-position);`
pub fn to_yellow_600() -> Style {
    Style::new(css!("--tw-gradient-to: #ca8a04 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-700`; css `--tw-gradient-to: #a16207 var(--tw-gradient-to-position);`
pub fn to_yellow_700() -> Style {
    Style::new(css!("--tw-gradient-to: #a16207 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-800`; css `--tw-gradient-to: #854d0e var(--tw-gradient-to-position);`
pub fn to_yellow_800() -> Style {
    Style::new(css!("--tw-gradient-to: #854d0e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-900`; css `--tw-gradient-to: #713f12 var(--tw-gradient-to-position);`
pub fn to_yellow_900() -> Style {
    Style::new(css!("--tw-gradient-to: #713f12 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-yellow-950`; css `--tw-gradient-to: #422006 var(--tw-gradient-to-position);`
pub fn to_yellow_950() -> Style {
    Style::new(css!("--tw-gradient-to: #422006 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-50`; css `--tw-gradient-to: #f7fee7 var(--tw-gradient-to-position);`
pub fn to_lime_50() -> Style {
    Style::new(css!("--tw-gradient-to: #f7fee7 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-100`; css `--tw-gradient-to: #ecfccb var(--tw-gradient-to-position);`
pub fn to_lime_100() -> Style {
    Style::new(css!("--tw-gradient-to: #ecfccb var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-200`; css `--tw-gradient-to: #d9f99d var(--tw-gradient-to-position);`
pub fn to_lime_200() -> Style {
    Style::new(css!("--tw-gradient-to: #d9f99d var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-300`; css `--tw-gradient-to: #bef264 var(--tw-gradient-to-position);`
pub fn to_lime_300() -> Style {
    Style::new(css!("--tw-gradient-to: #bef264 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-400`; css `--tw-gradient-to: #a3e635 var(--tw-gradient-to-position);`
pub fn to_lime_400() -> Style {
    Style::new(css!("--tw-gradient-to: #a3e635 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-500`; css `--tw-gradient-to: #84cc16 var(--tw-gradient-to-position);`
pub fn to_lime_500() -> Style {
    Style::new(css!("--tw-gradient-to: #84cc16 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-600`; css `--tw-gradient-to: #65a30d var(--tw-gradient-to-position);`
pub fn to_lime_600() -> Style {
    Style::new(css!("--tw-gradient-to: #65a30d var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-700`; css `--tw-gradient-to: #4d7c0f var(--tw-gradient-to-position);`
pub fn to_lime_700() -> Style {
    Style::new(css!("--tw-gradient-to: #4d7c0f var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-800`; css `--tw-gradient-to: #3f6212 var(--tw-gradient-to-position);`
pub fn to_lime_800() -> Style {
    Style::new(css!("--tw-gradient-to: #3f6212 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-900`; css `--tw-gradient-to: #365314 var(--tw-gradient-to-position);`
pub fn to_lime_900() -> Style {
    Style::new(css!("--tw-gradient-to: #365314 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-lime-950`; css `--tw-gradient-to: #1a2e05 var(--tw-gradient-to-position);`
pub fn to_lime_950() -> Style {
    Style::new(css!("--tw-gradient-to: #1a2e05 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-50`; css `--tw-gradient-to: #f0fdf4 var(--tw-gradient-to-position);`
pub fn to_green_50() -> Style {
    Style::new(css!("--tw-gradient-to: #f0fdf4 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-100`; css `--tw-gradient-to: #dcfce7 var(--tw-gradient-to-position);`
pub fn to_green_100() -> Style {
    Style::new(css!("--tw-gradient-to: #dcfce7 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-200`; css `--tw-gradient-to: #bbf7d0 var(--tw-gradient-to-position);`
pub fn to_green_200() -> Style {
    Style::new(css!("--tw-gradient-to: #bbf7d0 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-300`; css `--tw-gradient-to: #86efac var(--tw-gradient-to-position);`
pub fn to_green_300() -> Style {
    Style::new(css!("--tw-gradient-to: #86efac var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-400`; css `--tw-gradient-to: #4ade80 var(--tw-gradient-to-position);`
pub fn to_green_400() -> Style {
    Style::new(css!("--tw-gradient-to: #4ade80 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-500`; css `--tw-gradient-to: #22c55e var(--tw-gradient-to-position);`
pub fn to_green_500() -> Style {
    Style::new(css!("--tw-gradient-to: #22c55e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-600`; css `--tw-gradient-to: #16a34a var(--tw-gradient-to-position);`
pub fn to_green_600() -> Style {
    Style::new(css!("--tw-gradient-to: #16a34a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-700`; css `--tw-gradient-to: #15803d var(--tw-gradient-to-position);`
pub fn to_green_700() -> Style {
    Style::new(css!("--tw-gradient-to: #15803d var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-800`; css `--tw-gradient-to: #166534 var(--tw-gradient-to-position);`
pub fn to_green_800() -> Style {
    Style::new(css!("--tw-gradient-to: #166534 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-900`; css `--tw-gradient-to: #14532d var(--tw-gradient-to-position);`
pub fn to_green_900() -> Style {
    Style::new(css!("--tw-gradient-to: #14532d var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-green-950`; css `--tw-gradient-to: #052e16 var(--tw-gradient-to-position);`
pub fn to_green_950() -> Style {
    Style::new(css!("--tw-gradient-to: #052e16 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-50`; css `--tw-gradient-to: #ecfdf5 var(--tw-gradient-to-position);`
pub fn to_emerald_50() -> Style {
    Style::new(css!("--tw-gradient-to: #ecfdf5 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-100`; css `--tw-gradient-to: #d1fae5 var(--tw-gradient-to-position);`
pub fn to_emerald_100() -> Style {
    Style::new(css!("--tw-gradient-to: #d1fae5 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-200`; css `--tw-gradient-to: #a7f3d0 var(--tw-gradient-to-position);`
pub fn to_emerald_200() -> Style {
    Style::new(css!("--tw-gradient-to: #a7f3d0 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-300`; css `--tw-gradient-to: #6ee7b7 var(--tw-gradient-to-position);`
pub fn to_emerald_300() -> Style {
    Style::new(css!("--tw-gradient-to: #6ee7b7 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-400`; css `--tw-gradient-to: #34d399 var(--tw-gradient-to-position);`
pub fn to_emerald_400() -> Style {
    Style::new(css!("--tw-gradient-to: #34d399 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-500`; css `--tw-gradient-to: #10b981 var(--tw-gradient-to-position);`
pub fn to_emerald_500() -> Style {
    Style::new(css!("--tw-gradient-to: #10b981 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-600`; css `--tw-gradient-to: #059669 var(--tw-gradient-to-position);`
pub fn to_emerald_600() -> Style {
    Style::new(css!("--tw-gradient-to: #059669 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-700`; css `--tw-gradient-to: #047857 var(--tw-gradient-to-position);`
pub fn to_emerald_700() -> Style {
    Style::new(css!("--tw-gradient-to: #047857 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-800`; css `--tw-gradient-to: #065f46 var(--tw-gradient-to-position);`
pub fn to_emerald_800() -> Style {
    Style::new(css!("--tw-gradient-to: #065f46 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-900`; css `--tw-gradient-to: #064e3b var(--tw-gradient-to-position);`
pub fn to_emerald_900() -> Style {
    Style::new(css!("--tw-gradient-to: #064e3b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-emerald-950`; css `--tw-gradient-to: #022c22 var(--tw-gradient-to-position);`
pub fn to_emerald_950() -> Style {
    Style::new(css!("--tw-gradient-to: #022c22 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-50`; css `--tw-gradient-to: #f0fdfa var(--tw-gradient-to-position);`
pub fn to_teal_50() -> Style {
    Style::new(css!("--tw-gradient-to: #f0fdfa var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-100`; css `--tw-gradient-to: #ccfbf1 var(--tw-gradient-to-position);`
pub fn to_teal_100() -> Style {
    Style::new(css!("--tw-gradient-to: #ccfbf1 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-200`; css `--tw-gradient-to: #99f6e4 var(--tw-gradient-to-position);`
pub fn to_teal_200() -> Style {
    Style::new(css!("--tw-gradient-to: #99f6e4 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-300`; css `--tw-gradient-to: #5eead4 var(--tw-gradient-to-position);`
pub fn to_teal_300() -> Style {
    Style::new(css!("--tw-gradient-to: #5eead4 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-400`; css `--tw-gradient-to: #2dd4bf var(--tw-gradient-to-position);`
pub fn to_teal_400() -> Style {
    Style::new(css!("--tw-gradient-to: #2dd4bf var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-500`; css `--tw-gradient-to: #14b8a6 var(--tw-gradient-to-position);`
pub fn to_teal_500() -> Style {
    Style::new(css!("--tw-gradient-to: #14b8a6 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-600`; css `--tw-gradient-to: #0d9488 var(--tw-gradient-to-position);`
pub fn to_teal_600() -> Style {
    Style::new(css!("--tw-gradient-to: #0d9488 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-700`; css `--tw-gradient-to: #0f766e var(--tw-gradient-to-position);`
pub fn to_teal_700() -> Style {
    Style::new(css!("--tw-gradient-to: #0f766e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-800`; css `--tw-gradient-to: #115e59 var(--tw-gradient-to-position);`
pub fn to_teal_800() -> Style {
    Style::new(css!("--tw-gradient-to: #115e59 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-900`; css `--tw-gradient-to: #134e4a var(--tw-gradient-to-position);`
pub fn to_teal_900() -> Style {
    Style::new(css!("--tw-gradient-to: #134e4a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-teal-950`; css `--tw-gradient-to: #042f2e var(--tw-gradient-to-position);`
pub fn to_teal_950() -> Style {
    Style::new(css!("--tw-gradient-to: #042f2e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-50`; css `--tw-gradient-to: #ecfeff var(--tw-gradient-to-position);`
pub fn to_cyan_50() -> Style {
    Style::new(css!("--tw-gradient-to: #ecfeff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-100`; css `--tw-gradient-to: #cffafe var(--tw-gradient-to-position);`
pub fn to_cyan_100() -> Style {
    Style::new(css!("--tw-gradient-to: #cffafe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-200`; css `--tw-gradient-to: #a5f3fc var(--tw-gradient-to-position);`
pub fn to_cyan_200() -> Style {
    Style::new(css!("--tw-gradient-to: #a5f3fc var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-300`; css `--tw-gradient-to: #67e8f9 var(--tw-gradient-to-position);`
pub fn to_cyan_300() -> Style {
    Style::new(css!("--tw-gradient-to: #67e8f9 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-400`; css `--tw-gradient-to: #22d3ee var(--tw-gradient-to-position);`
pub fn to_cyan_400() -> Style {
    Style::new(css!("--tw-gradient-to: #22d3ee var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-500`; css `--tw-gradient-to: #06b6d4 var(--tw-gradient-to-position);`
pub fn to_cyan_500() -> Style {
    Style::new(css!("--tw-gradient-to: #06b6d4 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-600`; css `--tw-gradient-to: #0891b2 var(--tw-gradient-to-position);`
pub fn to_cyan_600() -> Style {
    Style::new(css!("--tw-gradient-to: #0891b2 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-700`; css `--tw-gradient-to: #0e7490 var(--tw-gradient-to-position);`
pub fn to_cyan_700() -> Style {
    Style::new(css!("--tw-gradient-to: #0e7490 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-800`; css `--tw-gradient-to: #155e75 var(--tw-gradient-to-position);`
pub fn to_cyan_800() -> Style {
    Style::new(css!("--tw-gradient-to: #155e75 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-900`; css `--tw-gradient-to: #164e63 var(--tw-gradient-to-position);`
pub fn to_cyan_900() -> Style {
    Style::new(css!("--tw-gradient-to: #164e63 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-cyan-950`; css `--tw-gradient-to: #083344 var(--tw-gradient-to-position);`
pub fn to_cyan_950() -> Style {
    Style::new(css!("--tw-gradient-to: #083344 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-50`; css `--tw-gradient-to: #f0f9ff var(--tw-gradient-to-position);`
pub fn to_sky_50() -> Style {
    Style::new(css!("--tw-gradient-to: #f0f9ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-100`; css `--tw-gradient-to: #e0f2fe var(--tw-gradient-to-position);`
pub fn to_sky_100() -> Style {
    Style::new(css!("--tw-gradient-to: #e0f2fe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-200`; css `--tw-gradient-to: #bae6fd var(--tw-gradient-to-position);`
pub fn to_sky_200() -> Style {
    Style::new(css!("--tw-gradient-to: #bae6fd var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-300`; css `--tw-gradient-to: #7dd3fc var(--tw-gradient-to-position);`
pub fn to_sky_300() -> Style {
    Style::new(css!("--tw-gradient-to: #7dd3fc var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-400`; css `--tw-gradient-to: #38bdf8 var(--tw-gradient-to-position);`
pub fn to_sky_400() -> Style {
    Style::new(css!("--tw-gradient-to: #38bdf8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-500`; css `--tw-gradient-to: #0ea5e9 var(--tw-gradient-to-position);`
pub fn to_sky_500() -> Style {
    Style::new(css!("--tw-gradient-to: #0ea5e9 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-600`; css `--tw-gradient-to: #0284c7 var(--tw-gradient-to-position);`
pub fn to_sky_600() -> Style {
    Style::new(css!("--tw-gradient-to: #0284c7 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-700`; css `--tw-gradient-to: #0369a1 var(--tw-gradient-to-position);`
pub fn to_sky_700() -> Style {
    Style::new(css!("--tw-gradient-to: #0369a1 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-800`; css `--tw-gradient-to: #075985 var(--tw-gradient-to-position);`
pub fn to_sky_800() -> Style {
    Style::new(css!("--tw-gradient-to: #075985 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-900`; css `--tw-gradient-to: #0c4a6e var(--tw-gradient-to-position);`
pub fn to_sky_900() -> Style {
    Style::new(css!("--tw-gradient-to: #0c4a6e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-sky-950`; css `--tw-gradient-to: #082f49 var(--tw-gradient-to-position);`
pub fn to_sky_950() -> Style {
    Style::new(css!("--tw-gradient-to: #082f49 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-50`; css `--tw-gradient-to: #eff6ff var(--tw-gradient-to-position);`
pub fn to_blue_50() -> Style {
    Style::new(css!("--tw-gradient-to: #eff6ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-100`; css `--tw-gradient-to: #dbeafe var(--tw-gradient-to-position);`
pub fn to_blue_100() -> Style {
    Style::new(css!("--tw-gradient-to: #dbeafe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-200`; css `--tw-gradient-to: #bfdbfe var(--tw-gradient-to-position);`
pub fn to_blue_200() -> Style {
    Style::new(css!("--tw-gradient-to: #bfdbfe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-300`; css `--tw-gradient-to: #93c5fd var(--tw-gradient-to-position);`
pub fn to_blue_300() -> Style {
    Style::new(css!("--tw-gradient-to: #93c5fd var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-400`; css `--tw-gradient-to: #60a5fa var(--tw-gradient-to-position);`
pub fn to_blue_400() -> Style {
    Style::new(css!("--tw-gradient-to: #60a5fa var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-500`; css `--tw-gradient-to: #3b82f6 var(--tw-gradient-to-position);`
pub fn to_blue_500() -> Style {
    Style::new(css!("--tw-gradient-to: #3b82f6 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-600`; css `--tw-gradient-to: #2563eb var(--tw-gradient-to-position);`
pub fn to_blue_600() -> Style {
    Style::new(css!("--tw-gradient-to: #2563eb var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-700`; css `--tw-gradient-to: #1d4ed8 var(--tw-gradient-to-position);`
pub fn to_blue_700() -> Style {
    Style::new(css!("--tw-gradient-to: #1d4ed8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-800`; css `--tw-gradient-to: #1e40af var(--tw-gradient-to-position);`
pub fn to_blue_800() -> Style {
    Style::new(css!("--tw-gradient-to: #1e40af var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-900`; css `--tw-gradient-to: #1e3a8a var(--tw-gradient-to-position);`
pub fn to_blue_900() -> Style {
    Style::new(css!("--tw-gradient-to: #1e3a8a var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-blue-950`; css `--tw-gradient-to: #172554 var(--tw-gradient-to-position);`
pub fn to_blue_950() -> Style {
    Style::new(css!("--tw-gradient-to: #172554 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-50`; css `--tw-gradient-to: #eef2ff var(--tw-gradient-to-position);`
pub fn to_indigo_50() -> Style {
    Style::new(css!("--tw-gradient-to: #eef2ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-100`; css `--tw-gradient-to: #e0e7ff var(--tw-gradient-to-position);`
pub fn to_indigo_100() -> Style {
    Style::new(css!("--tw-gradient-to: #e0e7ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-200`; css `--tw-gradient-to: #c7d2fe var(--tw-gradient-to-position);`
pub fn to_indigo_200() -> Style {
    Style::new(css!("--tw-gradient-to: #c7d2fe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-300`; css `--tw-gradient-to: #a5b4fc var(--tw-gradient-to-position);`
pub fn to_indigo_300() -> Style {
    Style::new(css!("--tw-gradient-to: #a5b4fc var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-400`; css `--tw-gradient-to: #818cf8 var(--tw-gradient-to-position);`
pub fn to_indigo_400() -> Style {
    Style::new(css!("--tw-gradient-to: #818cf8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-500`; css `--tw-gradient-to: #6366f1 var(--tw-gradient-to-position);`
pub fn to_indigo_500() -> Style {
    Style::new(css!("--tw-gradient-to: #6366f1 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-600`; css `--tw-gradient-to: #4f46e5 var(--tw-gradient-to-position);`
pub fn to_indigo_600() -> Style {
    Style::new(css!("--tw-gradient-to: #4f46e5 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-700`; css `--tw-gradient-to: #4338ca var(--tw-gradient-to-position);`
pub fn to_indigo_700() -> Style {
    Style::new(css!("--tw-gradient-to: #4338ca var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-800`; css `--tw-gradient-to: #3730a3 var(--tw-gradient-to-position);`
pub fn to_indigo_800() -> Style {
    Style::new(css!("--tw-gradient-to: #3730a3 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-900`; css `--tw-gradient-to: #312e81 var(--tw-gradient-to-position);`
pub fn to_indigo_900() -> Style {
    Style::new(css!("--tw-gradient-to: #312e81 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-indigo-950`; css `--tw-gradient-to: #1e1b4b var(--tw-gradient-to-position);`
pub fn to_indigo_950() -> Style {
    Style::new(css!("--tw-gradient-to: #1e1b4b var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-50`; css `--tw-gradient-to: #f5f3ff var(--tw-gradient-to-position);`
pub fn to_violet_50() -> Style {
    Style::new(css!("--tw-gradient-to: #f5f3ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-100`; css `--tw-gradient-to: #ede9fe var(--tw-gradient-to-position);`
pub fn to_violet_100() -> Style {
    Style::new(css!("--tw-gradient-to: #ede9fe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-200`; css `--tw-gradient-to: #ddd6fe var(--tw-gradient-to-position);`
pub fn to_violet_200() -> Style {
    Style::new(css!("--tw-gradient-to: #ddd6fe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-300`; css `--tw-gradient-to: #c4b5fd var(--tw-gradient-to-position);`
pub fn to_violet_300() -> Style {
    Style::new(css!("--tw-gradient-to: #c4b5fd var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-400`; css `--tw-gradient-to: #a78bfa var(--tw-gradient-to-position);`
pub fn to_violet_400() -> Style {
    Style::new(css!("--tw-gradient-to: #a78bfa var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-500`; css `--tw-gradient-to: #8b5cf6 var(--tw-gradient-to-position);`
pub fn to_violet_500() -> Style {
    Style::new(css!("--tw-gradient-to: #8b5cf6 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-600`; css `--tw-gradient-to: #7c3aed var(--tw-gradient-to-position);`
pub fn to_violet_600() -> Style {
    Style::new(css!("--tw-gradient-to: #7c3aed var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-700`; css `--tw-gradient-to: #6d28d9 var(--tw-gradient-to-position);`
pub fn to_violet_700() -> Style {
    Style::new(css!("--tw-gradient-to: #6d28d9 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-800`; css `--tw-gradient-to: #5b21b6 var(--tw-gradient-to-position);`
pub fn to_violet_800() -> Style {
    Style::new(css!("--tw-gradient-to: #5b21b6 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-900`; css `--tw-gradient-to: #4c1d95 var(--tw-gradient-to-position);`
pub fn to_violet_900() -> Style {
    Style::new(css!("--tw-gradient-to: #4c1d95 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-violet-950`; css `--tw-gradient-to: #2e1065 var(--tw-gradient-to-position);`
pub fn to_violet_950() -> Style {
    Style::new(css!("--tw-gradient-to: #2e1065 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-50`; css `--tw-gradient-to: #faf5ff var(--tw-gradient-to-position);`
pub fn to_purple_50() -> Style {
    Style::new(css!("--tw-gradient-to: #faf5ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-100`; css `--tw-gradient-to: #f3e8ff var(--tw-gradient-to-position);`
pub fn to_purple_100() -> Style {
    Style::new(css!("--tw-gradient-to: #f3e8ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-200`; css `--tw-gradient-to: #e9d5ff var(--tw-gradient-to-position);`
pub fn to_purple_200() -> Style {
    Style::new(css!("--tw-gradient-to: #e9d5ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-300`; css `--tw-gradient-to: #d8b4fe var(--tw-gradient-to-position);`
pub fn to_purple_300() -> Style {
    Style::new(css!("--tw-gradient-to: #d8b4fe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-400`; css `--tw-gradient-to: #c084fc var(--tw-gradient-to-position);`
pub fn to_purple_400() -> Style {
    Style::new(css!("--tw-gradient-to: #c084fc var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-500`; css `--tw-gradient-to: #a855f7 var(--tw-gradient-to-position);`
pub fn to_purple_500() -> Style {
    Style::new(css!("--tw-gradient-to: #a855f7 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-600`; css `--tw-gradient-to: #9333ea var(--tw-gradient-to-position);`
pub fn to_purple_600() -> Style {
    Style::new(css!("--tw-gradient-to: #9333ea var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-700`; css `--tw-gradient-to: #7e22ce var(--tw-gradient-to-position);`
pub fn to_purple_700() -> Style {
    Style::new(css!("--tw-gradient-to: #7e22ce var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-800`; css `--tw-gradient-to: #6b21a8 var(--tw-gradient-to-position);`
pub fn to_purple_800() -> Style {
    Style::new(css!("--tw-gradient-to: #6b21a8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-900`; css `--tw-gradient-to: #581c87 var(--tw-gradient-to-position);`
pub fn to_purple_900() -> Style {
    Style::new(css!("--tw-gradient-to: #581c87 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-purple-950`; css `--tw-gradient-to: #3b0764 var(--tw-gradient-to-position);`
pub fn to_purple_950() -> Style {
    Style::new(css!("--tw-gradient-to: #3b0764 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-50`; css `--tw-gradient-to: #fdf4ff var(--tw-gradient-to-position);`
pub fn to_fuchsia_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fdf4ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-100`; css `--tw-gradient-to: #fae8ff var(--tw-gradient-to-position);`
pub fn to_fuchsia_100() -> Style {
    Style::new(css!("--tw-gradient-to: #fae8ff var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-200`; css `--tw-gradient-to: #f5d0fe var(--tw-gradient-to-position);`
pub fn to_fuchsia_200() -> Style {
    Style::new(css!("--tw-gradient-to: #f5d0fe var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-300`; css `--tw-gradient-to: #f0abfc var(--tw-gradient-to-position);`
pub fn to_fuchsia_300() -> Style {
    Style::new(css!("--tw-gradient-to: #f0abfc var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-400`; css `--tw-gradient-to: #e879f9 var(--tw-gradient-to-position);`
pub fn to_fuchsia_400() -> Style {
    Style::new(css!("--tw-gradient-to: #e879f9 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-500`; css `--tw-gradient-to: #d946ef var(--tw-gradient-to-position);`
pub fn to_fuchsia_500() -> Style {
    Style::new(css!("--tw-gradient-to: #d946ef var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-600`; css `--tw-gradient-to: #c026d3 var(--tw-gradient-to-position);`
pub fn to_fuchsia_600() -> Style {
    Style::new(css!("--tw-gradient-to: #c026d3 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-700`; css `--tw-gradient-to: #a21caf var(--tw-gradient-to-position);`
pub fn to_fuchsia_700() -> Style {
    Style::new(css!("--tw-gradient-to: #a21caf var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-800`; css `--tw-gradient-to: #86198f var(--tw-gradient-to-position);`
pub fn to_fuchsia_800() -> Style {
    Style::new(css!("--tw-gradient-to: #86198f var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-900`; css `--tw-gradient-to: #701a75 var(--tw-gradient-to-position);`
pub fn to_fuchsia_900() -> Style {
    Style::new(css!("--tw-gradient-to: #701a75 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-fuchsia-950`; css `--tw-gradient-to: #4a044e var(--tw-gradient-to-position);`
pub fn to_fuchsia_950() -> Style {
    Style::new(css!("--tw-gradient-to: #4a044e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-50`; css `--tw-gradient-to: #fdf2f8 var(--tw-gradient-to-position);`
pub fn to_pink_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fdf2f8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-100`; css `--tw-gradient-to: #fce7f3 var(--tw-gradient-to-position);`
pub fn to_pink_100() -> Style {
    Style::new(css!("--tw-gradient-to: #fce7f3 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-200`; css `--tw-gradient-to: #fbcfe8 var(--tw-gradient-to-position);`
pub fn to_pink_200() -> Style {
    Style::new(css!("--tw-gradient-to: #fbcfe8 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-300`; css `--tw-gradient-to: #f9a8d4 var(--tw-gradient-to-position);`
pub fn to_pink_300() -> Style {
    Style::new(css!("--tw-gradient-to: #f9a8d4 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-400`; css `--tw-gradient-to: #f472b6 var(--tw-gradient-to-position);`
pub fn to_pink_400() -> Style {
    Style::new(css!("--tw-gradient-to: #f472b6 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-500`; css `--tw-gradient-to: #ec4899 var(--tw-gradient-to-position);`
pub fn to_pink_500() -> Style {
    Style::new(css!("--tw-gradient-to: #ec4899 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-600`; css `--tw-gradient-to: #db2777 var(--tw-gradient-to-position);`
pub fn to_pink_600() -> Style {
    Style::new(css!("--tw-gradient-to: #db2777 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-700`; css `--tw-gradient-to: #be185d var(--tw-gradient-to-position);`
pub fn to_pink_700() -> Style {
    Style::new(css!("--tw-gradient-to: #be185d var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-800`; css `--tw-gradient-to: #9d174d var(--tw-gradient-to-position);`
pub fn to_pink_800() -> Style {
    Style::new(css!("--tw-gradient-to: #9d174d var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-900`; css `--tw-gradient-to: #831843 var(--tw-gradient-to-position);`
pub fn to_pink_900() -> Style {
    Style::new(css!("--tw-gradient-to: #831843 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-pink-950`; css `--tw-gradient-to: #500724 var(--tw-gradient-to-position);`
pub fn to_pink_950() -> Style {
    Style::new(css!("--tw-gradient-to: #500724 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-50`; css `--tw-gradient-to: #fff1f2 var(--tw-gradient-to-position);`
pub fn to_rose_50() -> Style {
    Style::new(css!("--tw-gradient-to: #fff1f2 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-100`; css `--tw-gradient-to: #ffe4e6 var(--tw-gradient-to-position);`
pub fn to_rose_100() -> Style {
    Style::new(css!("--tw-gradient-to: #ffe4e6 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-200`; css `--tw-gradient-to: #fecdd3 var(--tw-gradient-to-position);`
pub fn to_rose_200() -> Style {
    Style::new(css!("--tw-gradient-to: #fecdd3 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-300`; css `--tw-gradient-to: #fda4af var(--tw-gradient-to-position);`
pub fn to_rose_300() -> Style {
    Style::new(css!("--tw-gradient-to: #fda4af var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-400`; css `--tw-gradient-to: #fb7185 var(--tw-gradient-to-position);`
pub fn to_rose_400() -> Style {
    Style::new(css!("--tw-gradient-to: #fb7185 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-500`; css `--tw-gradient-to: #f43f5e var(--tw-gradient-to-position);`
pub fn to_rose_500() -> Style {
    Style::new(css!("--tw-gradient-to: #f43f5e var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-600`; css `--tw-gradient-to: #e11d48 var(--tw-gradient-to-position);`
pub fn to_rose_600() -> Style {
    Style::new(css!("--tw-gradient-to: #e11d48 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-700`; css `--tw-gradient-to: #be123c var(--tw-gradient-to-position);`
pub fn to_rose_700() -> Style {
    Style::new(css!("--tw-gradient-to: #be123c var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-800`; css `--tw-gradient-to: #9f1239 var(--tw-gradient-to-position);`
pub fn to_rose_800() -> Style {
    Style::new(css!("--tw-gradient-to: #9f1239 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-900`; css `--tw-gradient-to: #881337 var(--tw-gradient-to-position);`
pub fn to_rose_900() -> Style {
    Style::new(css!("--tw-gradient-to: #881337 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-rose-950`; css `--tw-gradient-to: #4c0519 var(--tw-gradient-to-position);`
pub fn to_rose_950() -> Style {
    Style::new(css!("--tw-gradient-to: #4c0519 var(--tw-gradient-to-position);")).unwrap()
}

/// Tailwind `to-0%`; css `--tw-gradient-to-position: 0%;`
pub fn to_0p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 0%;")).unwrap()
}

/// Tailwind `to-5%`; css `--tw-gradient-to-position: 5%;`
pub fn to_5p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 5%;")).unwrap()
}

/// Tailwind `to-10%`; css `--tw-gradient-to-position: 10%;`
pub fn to_10p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 10%;")).unwrap()
}

/// Tailwind `to-15%`; css `--tw-gradient-to-position: 15%;`
pub fn to_15p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 15%;")).unwrap()
}

/// Tailwind `to-20%`; css `--tw-gradient-to-position: 20%;`
pub fn to_20p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 20%;")).unwrap()
}

/// Tailwind `to-25%`; css `--tw-gradient-to-position: 25%;`
pub fn to_25p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 25%;")).unwrap()
}

/// Tailwind `to-30%`; css `--tw-gradient-to-position: 30%;`
pub fn to_30p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 30%;")).unwrap()
}

/// Tailwind `to-35%`; css `--tw-gradient-to-position: 35%;`
pub fn to_35p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 35%;")).unwrap()
}

/// Tailwind `to-40%`; css `--tw-gradient-to-position: 40%;`
pub fn to_40p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 40%;")).unwrap()
}

/// Tailwind `to-45%`; css `--tw-gradient-to-position: 45%;`
pub fn to_45p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 45%;")).unwrap()
}

/// Tailwind `to-50%`; css `--tw-gradient-to-position: 50%;`
pub fn to_50p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 50%;")).unwrap()
}

/// Tailwind `to-55%`; css `--tw-gradient-to-position: 55%;`
pub fn to_55p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 55%;")).unwrap()
}

/// Tailwind `to-60%`; css `--tw-gradient-to-position: 60%;`
pub fn to_60p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 60%;")).unwrap()
}

/// Tailwind `to-65%`; css `--tw-gradient-to-position: 65%;`
pub fn to_65p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 65%;")).unwrap()
}

/// Tailwind `to-70%`; css `--tw-gradient-to-position: 70%;`
pub fn to_70p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 70%;")).unwrap()
}

/// Tailwind `to-75%`; css `--tw-gradient-to-position: 75%;`
pub fn to_75p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 75%;")).unwrap()
}

/// Tailwind `to-80%`; css `--tw-gradient-to-position: 80%;`
pub fn to_80p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 80%;")).unwrap()
}

/// Tailwind `to-85%`; css `--tw-gradient-to-position: 85%;`
pub fn to_85p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 85%;")).unwrap()
}

/// Tailwind `to-90%`; css `--tw-gradient-to-position: 90%;`
pub fn to_90p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 90%;")).unwrap()
}

/// Tailwind `to-95%`; css `--tw-gradient-to-position: 95%;`
pub fn to_95p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 95%;")).unwrap()
}

/// Tailwind `to-100%`; css `--tw-gradient-to-position: 100%;`
pub fn to_100p() -> Style {
    Style::new(css!("--tw-gradient-to-position: 100%;")).unwrap()
}

