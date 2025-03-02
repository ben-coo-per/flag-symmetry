## Ever wonder how many of the world flags are symmetrical?

During a long car ride, a friend and I were joking about how [the recent flag controversy with Justice Samuel Alito's wife](https://apnews.com/article/justice-samuel-alito-upsidedown-flag-trump-jan6-f5809b9fd3be19b2359907f7b16651e5) wouldn't have happened if we had a flag like 🇨🇭Switzerland's, since there's no way to hang it upside down.

This got me thinking more broadly about flag symmetry. Plus, I've been wanting an excuse to learn Rust, so voilà!

The full results can be viewed in `results.csv`

### In summary:

[Instagram Post](https://www.instagram.com/p/C9F03HevOH7)

```
+----------------------------------------+-----+
| 🪩 Flags with full symmetry            | 14  |
+----------------------------------------+-----+
| ↕️ Flags with horizontal line symmetry  | 23  |
+----------------------------------------+-----+
| ↔️ Flags with vertical line symmetry    | 42  |
+----------------------------------------+-----+
| ❌ Flags with no symmetry              | 173 |
+----------------------------------------+-----+

Full symmetry flags: 🇵🇪🇯🇲🇯🇵🇹🇭🇫🇲🇱🇻🇨🇭🇧🇼🇮🇱🇦🇹🇲🇰🇬🇬🇳🇬🇬🇪
Horizontal line symmetry flags: 🇬🇾🇦🇽🇵🇼🇬🇳🇲🇫🇫🇴🇮🇪🇫🇷🇧🇪🇫🇮🇹🇩🏳️🇨🇮🇸🇪🇩🇰🇧🇭🇸🇯🇲🇱🇮🇸🇳🇴🇶🇦🇷🇴🇮🇹
Vertical line symmetry flags: 🇬🇦🇸🇷🇷🇪🇮🇳🇸🇴🇲🇦🇱🇹🇷🇺🇹🇯🇭🇺🇩🇪🇲🇲🇻🇳🇸🇾🇳🇱🇧🇬🇻🇪🇸🇱🇲🇨🇱🇺🇭🇳🇦🇱🇨🇴🇲🇺🇮🇩🇬🇭🇬🇲🇧🇮🇲🇷🇪🇪🇺🇦🇱🇦🇦🇬🇲🇴🇪🇺🇧🇧🇱🇨🇵🇱🇦🇲🇨🇦🇳🇪🇾🇪
No symmetry flags: 🇿🇼🇹🇨🇭🇰🇻🇨🇦🇸🇳🇺🇮🇴🇱🇾🇲🇶🇧🇲🇨🇰🇧🇿🇸🇬🇪🇬🇵🇦🇿🇦🇰🇬🇯🇪🇼🇫🇬🇮🇳🇷🇯🇴🇵🇭🇺🇿🇧🇩🇪🇸🇭🇷🇸🇧🇪🇭🇰🇷🇹🇬🇲🇳🇪🇹🇳🇮🇹🇹🇳🇫🇨🇼🇸🇸🇧🇯🇱🇰🇧🇴🇦🇮🇵🇬🇨🇻🇰🇵🇮🇷🇸🇮🇦🇺🏳️🇼🇸🇧🇸🏳️🇧🇫🇲🇭🇰🇿🇺🇳🇳🇨🇵🇹🇨🇱🇬🇧🇨🇬🇬🇫🇨🇩🇬🇵🇭🇹🇲🇪🇦🇼🇲🇽🇷🇸🇹🇲🇧🇦🇲🇹🇱🇮🇰🇮🇫🇰🇵🇳🇹🇰🇳🇦🇦🇿🇨🇷🇨🇿🇬🇱🇸🇽🇦🇪🇨🇫🏳️🇰🇲🇹🇷🇸🇿🇬🇹🇲🇬🇨🇳🇦🇷🇺🇸🇧🇱🇳🇵🇰🇼🇫🇯🏳️🇰🇾🇲🇾🇬🇸🇬🇺🇾🇹🇵🇫🇸🇹🇨🇾🇵🇸🇹🇳🇺🇾🇮🇲🇺🇬🇹🇱🇧🇳🏳️🇮🇶🇲🇩🇧🇹🇧🇾🇪🇨🇸🇩🇸🇦🇴🇲🇱🇧🇦🇶🇰🇳🇰🇪🇲🇼🇬🇼🇪🇷🇬🇶🇩🇲🇰🇭🇸🇲🇵🇰🇽🇰🇩🇯🇹🇼🇷🇼🇨🇲🇦🇩🇸🇭🇻🇬🇹🇴🇦🇴🇬🇷🇸🇻🇲🇿🇨🇺🇸🇨🇻🇦🇵🇾🇸🇰🇿🇲🇲🇸🇹🇻🇹🇿🇵🇷🇻🇮🇩🇴🇩🇿🇲🇵🇳🇿🇱🇷🇻🇺🇱🇸🇬🇩🇸🇳🇲🇻🇧🇷🇦🇫🏳️🇵🇲
```

\*flags that do not have an emoji for are shown as 🏳️
