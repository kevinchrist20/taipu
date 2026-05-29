import { reactive } from "vue";
import { StorageService } from "../services";
import { User } from "../types/bindings";

export const SessionStore = reactive({
    user: StorageService.get<User>("user") || null,
  theme: "LIGHT",
  
    setUser(user: User) {
      this.user = user;
      StorageService.set("user", user);
    },
  
    setTheme(theme: string) {
      this.theme = theme;
    },

    setUserPreferences(language: string, lessonDifficulty: string) {
      if (!this.user) return;

      this.user = {
        ...this.user,
        language,
        lessonDifficulty,
      };
      StorageService.set("user", this.user);
    },
  
    clearUser() {
      this.user = null;
      StorageService.remove("user");
    },
  
    clearTheme() {
      this.theme = "LIGHT";
    },
  
    clearAll() {
      this.user = null;
      this.theme = "LIGHT";
      StorageService.clear();
    },
  });