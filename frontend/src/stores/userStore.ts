import type { User } from "oidc-client-ts";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useUserStore = defineStore("user", () => {
  const currentUser = ref<User | null>(null);

  const setUser = (user: User | null) => {
    currentUser.value = user;
  };

  const clearUser = () => {
    currentUser.value = null;
  };

  return {
    currentUser,
    setUser,
    clearUser,
  }
});
