import { reactive } from "vue";
import { StorageService } from "../services";

export const SessionStore = reactive({
    user: StorageService.get<Record<string, any>>("user") || null,
    theme: StorageService.get<string>("theme") ?? "LIGHT",
  
    setUser(user: Record<string, any>) {
      this.user = user;
      StorageService.set("user", user);
    },
  
    setTheme(theme: string) {
      this.theme = theme;
      StorageService.set("theme", theme);
    },
  
    clearUser() {
      this.user = null;
      StorageService.remove("user");
    },
  
    clearTheme() {
      this.theme = "LIGHT";
      StorageService.set("theme", "LIGHT");
    },
  
    clearAll() {
      this.user = null;
      this.theme = "LIGHT";
      StorageService.clear();
    },
  });