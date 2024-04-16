use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "divide-inherit " => "border-color: inherit;",
    "divide-current " => "border-color: currentColor;",
    "divide-transparent " => "border-color: transparent;",
    "divide-black " => "border-color: rgb(0 0 0);",
    "divide-white " => "border-color: rgb(255 255 255);",
    "divide-slate-50 " => "border-color: rgb(248 250 252);",
    "divide-slate-100 " => "border-color: rgb(241 245 249);",
    "divide-slate-200 " => "border-color: rgb(226 232 240);",
    "divide-slate-300 " => "border-color: rgb(203 213 225);",
    "divide-slate-400 " => "border-color: rgb(148 163 184);",
    "divide-slate-500 " => "border-color: rgb(100 116 139);",
    "divide-slate-600 " => "border-color: rgb(71 85 105);",
    "divide-slate-700 " => "border-color: rgb(51 65 85);",
    "divide-slate-800 " => "border-color: rgb(30 41 59);",
    "divide-slate-900 " => "border-color: rgb(15 23 42);",
    "divide-slate-950 " => "border-color: rgb(2 6 23);",
    "divide-gray-50 " => "border-color: rgb(249 250 251);",
    "divide-gray-100 " => "border-color: rgb(243 244 246);",
    "divide-gray-200 " => "border-color: rgb(229 231 235);",
    "divide-gray-300 " => "border-color: rgb(209 213 219);",
    "divide-gray-400 " => "border-color: rgb(156 163 175);",
    "divide-gray-500 " => "border-color: rgb(107 114 128);",
    "divide-gray-600 " => "border-color: rgb(75 85 99);",
    "divide-gray-700 " => "border-color: rgb(55 65 81);",
    "divide-gray-800 " => "border-color: rgb(31 41 55);",
    "divide-gray-900 " => "border-color: rgb(17 24 39);",
    "divide-gray-950 " => "border-color: rgb(3 7 18);",
    "divide-zinc-50 " => "border-color: rgb(250 250 250);",
    "divide-zinc-100 " => "border-color: rgb(244 244 245);",
    "divide-zinc-200 " => "border-color: rgb(228 228 231);",
    "divide-zinc-300 " => "border-color: rgb(212 212 216);",
    "divide-zinc-400 " => "border-color: rgb(161 161 170);",
    "divide-zinc-500 " => "border-color: rgb(113 113 122);",
    "divide-zinc-600 " => "border-color: rgb(82 82 91);",
    "divide-zinc-700 " => "border-color: rgb(63 63 70);",
    "divide-zinc-800 " => "border-color: rgb(39 39 42);",
    "divide-zinc-900 " => "border-color: rgb(24 24 27);",
    "divide-zinc-950 " => "border-color: rgb(9 9 11);",
    "divide-neutral-50 " => "border-color: rgb(250 250 250);",
    "divide-neutral-100 " => "border-color: rgb(245 245 245);",
    "divide-neutral-200 " => "border-color: rgb(229 229 229);",
    "divide-neutral-300 " => "border-color: rgb(212 212 212);",
    "divide-neutral-400 " => "border-color: rgb(163 163 163);",
    "divide-neutral-500 " => "border-color: rgb(115 115 115);",
    "divide-neutral-600 " => "border-color: rgb(82 82 82);",
    "divide-neutral-700 " => "border-color: rgb(64 64 64);",
    "divide-neutral-800 " => "border-color: rgb(38 38 38);",
    "divide-neutral-900 " => "border-color: rgb(23 23 23);",
    "divide-neutral-950 " => "border-color: rgb(10 10 10);",
    "divide-stone-50 " => "border-color: rgb(250 250 249);",
    "divide-stone-100 " => "border-color: rgb(245 245 244);",
    "divide-stone-200 " => "border-color: rgb(231 229 228);",
    "divide-stone-300 " => "border-color: rgb(214 211 209);",
    "divide-stone-400 " => "border-color: rgb(168 162 158);",
    "divide-stone-500 " => "border-color: rgb(120 113 108);",
    "divide-stone-600 " => "border-color: rgb(87 83 78);",
    "divide-stone-700 " => "border-color: rgb(68 64 60);",
    "divide-stone-800 " => "border-color: rgb(41 37 36);",
    "divide-stone-900 " => "border-color: rgb(28 25 23);",
    "divide-stone-950 " => "border-color: rgb(12 10 9);",
    "divide-red-50 " => "border-color: rgb(254 242 242);",
    "divide-red-100 " => "border-color: rgb(254 226 226);",
    "divide-red-200 " => "border-color: rgb(254 202 202);",
    "divide-red-300 " => "border-color: rgb(252 165 165);",
    "divide-red-400 " => "border-color: rgb(248 113 113);",
    "divide-red-500 " => "border-color: rgb(239 68 68);",
    "divide-red-600 " => "border-color: rgb(220 38 38);",
    "divide-red-700 " => "border-color: rgb(185 28 28);",
    "divide-red-800 " => "border-color: rgb(153 27 27);",
    "divide-red-900 " => "border-color: rgb(127 29 29);",
    "divide-red-950 " => "border-color: rgb(69 10 10);",
    "divide-orange-50 " => "border-color: rgb(255 247 237);",
    "divide-orange-100 " => "border-color: rgb(255 237 213);",
    "divide-orange-200 " => "border-color: rgb(254 215 170);",
    "divide-orange-300 " => "border-color: rgb(253 186 116);",
    "divide-orange-400 " => "border-color: rgb(251 146 60);",
    "divide-orange-500 " => "border-color: rgb(249 115 22);",
    "divide-orange-600 " => "border-color: rgb(234 88 12);",
    "divide-orange-700 " => "border-color: rgb(194 65 12);",
    "divide-orange-800 " => "border-color: rgb(154 52 18);",
    "divide-orange-900 " => "border-color: rgb(124 45 18);",
    "divide-orange-950 " => "border-color: rgb(67 20 7);",
    "divide-amber-50 " => "border-color: rgb(255 251 235);",
    "divide-amber-100 " => "border-color: rgb(254 243 199);",
    "divide-amber-200 " => "border-color: rgb(253 230 138);",
    "divide-amber-300 " => "border-color: rgb(252 211 77);",
    "divide-amber-400 " => "border-color: rgb(251 191 36);",
    "divide-amber-500 " => "border-color: rgb(245 158 11);",
    "divide-amber-600 " => "border-color: rgb(217 119 6);",
    "divide-amber-700 " => "border-color: rgb(180 83 9);",
    "divide-amber-800 " => "border-color: rgb(146 64 14);",
    "divide-amber-900 " => "border-color: rgb(120 53 15);",
    "divide-amber-950 " => "border-color: rgb(69 26 3);",
    "divide-yellow-50 " => "border-color: rgb(254 252 232);",
    "divide-yellow-100 " => "border-color: rgb(254 249 195);",
    "divide-yellow-200 " => "border-color: rgb(254 240 138);",
    "divide-yellow-300 " => "border-color: rgb(253 224 71);",
    "divide-yellow-400 " => "border-color: rgb(250 204 21);",
    "divide-yellow-500 " => "border-color: rgb(234 179 8);",
    "divide-yellow-600 " => "border-color: rgb(202 138 4);",
    "divide-yellow-700 " => "border-color: rgb(161 98 7);",
    "divide-yellow-800 " => "border-color: rgb(133 77 14);",
    "divide-yellow-900 " => "border-color: rgb(113 63 18);",
    "divide-yellow-950 " => "border-color: rgb(66 32 6);",
    "divide-lime-50 " => "border-color: rgb(247 254 231);",
    "divide-lime-100 " => "border-color: rgb(236 252 203);",
    "divide-lime-200 " => "border-color: rgb(217 249 157);",
    "divide-lime-300 " => "border-color: rgb(190 242 100);",
    "divide-lime-400 " => "border-color: rgb(163 230 53);",
    "divide-lime-500 " => "border-color: rgb(132 204 22);",
    "divide-lime-600 " => "border-color: rgb(101 163 13);",
    "divide-lime-700 " => "border-color: rgb(77 124 15);",
    "divide-lime-800 " => "border-color: rgb(63 98 18);",
    "divide-lime-900 " => "border-color: rgb(54 83 20);",
    "divide-lime-950 " => "border-color: rgb(26 46 5);",
    "divide-green-50 " => "border-color: rgb(240 253 244);",
    "divide-green-100 " => "border-color: rgb(220 252 231);",
    "divide-green-200 " => "border-color: rgb(187 247 208);",
    "divide-green-300 " => "border-color: rgb(134 239 172);",
    "divide-green-400 " => "border-color: rgb(74 222 128);",
    "divide-green-500 " => "border-color: rgb(34 197 94);",
    "divide-green-600 " => "border-color: rgb(22 163 74);",
    "divide-green-700 " => "border-color: rgb(21 128 61);",
    "divide-green-800 " => "border-color: rgb(22 101 52);",
    "divide-green-900 " => "border-color: rgb(20 83 45);",
    "divide-green-950 " => "border-color: rgb(5 46 22);",
    "divide-emerald-50 " => "border-color: rgb(236 253 245);",
    "divide-emerald-100 " => "border-color: rgb(209 250 229);",
    "divide-emerald-200 " => "border-color: rgb(167 243 208);",
    "divide-emerald-300 " => "border-color: rgb(110 231 183);",
    "divide-emerald-400 " => "border-color: rgb(52 211 153);",
    "divide-emerald-500 " => "border-color: rgb(16 185 129);",
    "divide-emerald-600 " => "border-color: rgb(5 150 105);",
    "divide-emerald-700 " => "border-color: rgb(4 120 87);",
    "divide-emerald-800 " => "border-color: rgb(6 95 70);",
    "divide-emerald-900 " => "border-color: rgb(6 78 59);",
    "divide-emerald-950 " => "border-color: rgb(2 44 34);",
    "divide-teal-50 " => "border-color: rgb(240 253 250);",
    "divide-teal-100 " => "border-color: rgb(204 251 241);",
    "divide-teal-200 " => "border-color: rgb(153 246 228);",
    "divide-teal-300 " => "border-color: rgb(94 234 212);",
    "divide-teal-400 " => "border-color: rgb(45 212 191);",
    "divide-teal-500 " => "border-color: rgb(20 184 166);",
    "divide-teal-600 " => "border-color: rgb(13 148 136);",
    "divide-teal-700 " => "border-color: rgb(15 118 110);",
    "divide-teal-800 " => "border-color: rgb(17 94 89);",
    "divide-teal-900 " => "border-color: rgb(19 78 74);",
    "divide-teal-950 " => "border-color: rgb(4 47 46);",
    "divide-cyan-50 " => "border-color: rgb(236 254 255);",
    "divide-cyan-100 " => "border-color: rgb(207 250 254);",
    "divide-cyan-200 " => "border-color: rgb(165 243 252);",
    "divide-cyan-300 " => "border-color: rgb(103 232 249);",
    "divide-cyan-400 " => "border-color: rgb(34 211 238);",
    "divide-cyan-500 " => "border-color: rgb(6 182 212);",
    "divide-cyan-600 " => "border-color: rgb(8 145 178);",
    "divide-cyan-700 " => "border-color: rgb(14 116 144);",
    "divide-cyan-800 " => "border-color: rgb(21 94 117);",
    "divide-cyan-900 " => "border-color: rgb(22 78 99);",
    "divide-cyan-950 " => "border-color: rgb(8 51 68);",
    "divide-sky-50 " => "border-color: rgb(240 249 255);",
    "divide-sky-100 " => "border-color: rgb(224 242 254);",
    "divide-sky-200 " => "border-color: rgb(186 230 253);",
    "divide-sky-300 " => "border-color: rgb(125 211 252);",
    "divide-sky-400 " => "border-color: rgb(56 189 248);",
    "divide-sky-500 " => "border-color: rgb(14 165 233);",
    "divide-sky-600 " => "border-color: rgb(2 132 199);",
    "divide-sky-700 " => "border-color: rgb(3 105 161);",
    "divide-sky-800 " => "border-color: rgb(7 89 133);",
    "divide-sky-900 " => "border-color: rgb(12 74 110);",
    "divide-sky-950 " => "border-color: rgb(8 47 73);",
    "divide-blue-50 " => "border-color: rgb(239 246 255);",
    "divide-blue-100 " => "border-color: rgb(219 234 254);",
    "divide-blue-200 " => "border-color: rgb(191 219 254);",
    "divide-blue-300 " => "border-color: rgb(147 197 253);",
    "divide-blue-400 " => "border-color: rgb(96 165 250);",
    "divide-blue-500 " => "border-color: rgb(59 130 246);",
    "divide-blue-600 " => "border-color: rgb(37 99 235);",
    "divide-blue-700 " => "border-color: rgb(29 78 216);",
    "divide-blue-800 " => "border-color: rgb(30 64 175);",
    "divide-blue-900 " => "border-color: rgb(30 58 138);",
    "divide-blue-950 " => "border-color: rgb(23 37 84);",
    "divide-indigo-50 " => "border-color: rgb(238 242 255);",
    "divide-indigo-100 " => "border-color: rgb(224 231 255);",
    "divide-indigo-200 " => "border-color: rgb(199 210 254);",
    "divide-indigo-300 " => "border-color: rgb(165 180 252);",
    "divide-indigo-400 " => "border-color: rgb(129 140 248);",
    "divide-indigo-500 " => "border-color: rgb(99 102 241);",
    "divide-indigo-600 " => "border-color: rgb(79 70 229);",
    "divide-indigo-700 " => "border-color: rgb(67 56 202);",
    "divide-indigo-800 " => "border-color: rgb(55 48 163);",
    "divide-indigo-900 " => "border-color: rgb(49 46 129);",
    "divide-indigo-950 " => "border-color: rgb(30 27 75);",
    "divide-violet-50 " => "border-color: rgb(245 243 255);",
    "divide-violet-100 " => "border-color: rgb(237 233 254);",
    "divide-violet-200 " => "border-color: rgb(221 214 254);",
    "divide-violet-300 " => "border-color: rgb(196 181 253);",
    "divide-violet-400 " => "border-color: rgb(167 139 250);",
    "divide-violet-500 " => "border-color: rgb(139 92 246);",
    "divide-violet-600 " => "border-color: rgb(124 58 237);",
    "divide-violet-700 " => "border-color: rgb(109 40 217);",
    "divide-violet-800 " => "border-color: rgb(91 33 182);",
    "divide-violet-900 " => "border-color: rgb(76 29 149);",
    "divide-violet-950 " => "border-color: rgb(46 16 101);",
    "divide-purple-50 " => "border-color: rgb(250 245 255);",
    "divide-purple-100 " => "border-color: rgb(243 232 255);",
    "divide-purple-200 " => "border-color: rgb(233 213 255);",
    "divide-purple-300 " => "border-color: rgb(216 180 254);",
    "divide-purple-400 " => "border-color: rgb(192 132 252);",
    "divide-purple-500 " => "border-color: rgb(168 85 247);",
    "divide-purple-600 " => "border-color: rgb(147 51 234);",
    "divide-purple-700 " => "border-color: rgb(126 34 206);",
    "divide-purple-800 " => "border-color: rgb(107 33 168);",
    "divide-purple-900 " => "border-color: rgb(88 28 135);",
    "divide-purple-950 " => "border-color: rgb(59 7 100);",
    "divide-fuchsia-50 " => "border-color: rgb(253 244 255);",
    "divide-fuchsia-100 " => "border-color: rgb(250 232 255);",
    "divide-fuchsia-200 " => "border-color: rgb(245 208 254);",
    "divide-fuchsia-300 " => "border-color: rgb(240 171 252);",
    "divide-fuchsia-400 " => "border-color: rgb(232 121 249);",
    "divide-fuchsia-500 " => "border-color: rgb(217 70 239);",
    "divide-fuchsia-600 " => "border-color: rgb(192 38 211);",
    "divide-fuchsia-700 " => "border-color: rgb(162 28 175);",
    "divide-fuchsia-800 " => "border-color: rgb(134 25 143);",
    "divide-fuchsia-900 " => "border-color: rgb(112 26 117);",
    "divide-fuchsia-950 " => "border-color: rgb(74 4 78);",
    "divide-pink-50 " => "border-color: rgb(253 242 248);",
    "divide-pink-100 " => "border-color: rgb(252 231 243);",
    "divide-pink-200 " => "border-color: rgb(251 207 232);",
    "divide-pink-300 " => "border-color: rgb(249 168 212);",
    "divide-pink-400 " => "border-color: rgb(244 114 182);",
    "divide-pink-500 " => "border-color: rgb(236 72 153);",
    "divide-pink-600 " => "border-color: rgb(219 39 119);",
    "divide-pink-700 " => "border-color: rgb(190 24 93);",
    "divide-pink-800 " => "border-color: rgb(157 23 77);",
    "divide-pink-900 " => "border-color: rgb(131 24 67);",
    "divide-pink-950 " => "border-color: rgb(80 7 36);",
    "divide-rose-50 " => "border-color: rgb(255 241 242);",
    "divide-rose-100 " => "border-color: rgb(255 228 230);",
    "divide-rose-200 " => "border-color: rgb(254 205 211);",
    "divide-rose-300 " => "border-color: rgb(253 164 175);",
    "divide-rose-400 " => "border-color: rgb(251 113 133);",
    "divide-rose-500 " => "border-color: rgb(244 63 94);",
    "divide-rose-600 " => "border-color: rgb(225 29 72);",
    "divide-rose-700 " => "border-color: rgb(190 18 60);",
    "divide-rose-800 " => "border-color: rgb(159 18 57);",
    "divide-rose-900 " => "border-color: rgb(136 19 55);",
    "divide-rose-950 " => "border-color: rgb(76 5 25);",
};