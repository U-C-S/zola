import { zolaBuild, zolaCheck, zolaInit, zolaServe } from "./index.js";

// zolaBuild(".", "config.toml", "/",  "./public", false, false);
// zolaCheck(".", "config.toml", "/", "/", false, false);
// zolaInit("abcd", false);
zolaServe("/home/greenpenguin/code/zola-npm/zola/test_site/", "127.0.0.1", 8000, null, "/", "config.toml", false, false, true, false);
// zolaServe(".", "127.0.0.1", 8000, null, "/", "config.toml", false, false, true, false);