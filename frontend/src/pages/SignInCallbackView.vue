<template>
  <div>Signing in...</div>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/userStore.ts";
import { UserManager } from "oidc-client-ts";
import { inject, onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const userManager = inject<UserManager>("userManager")!;
const userStore = useUserStore();

onMounted(async () => {
  try {
    const user = await userManager.signinRedirectCallback();
    userStore.setUser(user);
    await router.push("/");
  } catch (e) {
    console.error("Callback failed:", e);
    await router.push("/");
  }
});
</script>
