<template>
  <div>
    <button @click="signOut" v-if="userStore.currentUser">Sign Out</button>
    <button @click="signIn" v-else>Sign In</button>
  </div>

  <main>
    <RouterView />
  </main>
</template>

<script setup lang="ts">
import { signOutRedirect } from "@/auth.ts";
import { useUserStore } from "@/stores/userStore.ts";
import { UserManager } from "oidc-client-ts";
import { inject } from "vue";

const userManager = inject<UserManager>("userManager")!;
const userStore = useUserStore();

const signIn = async () => {
  await userManager.signinRedirect();
};

const signOut = async () => {
  await signOutRedirect();
};

</script>
