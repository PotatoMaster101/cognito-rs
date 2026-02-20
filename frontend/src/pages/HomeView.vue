<template>
  <div v-if="userStore.currentUser">
    <button @click="callBackend">Call Backend</button>
    <p>User: {{ userStore.currentUser.profile.email }}</p>
    <p>Access Token: {{ userStore.currentUser.access_token }}</p>
    <p>ID Token: {{ userStore.currentUser.id_token }}</p>
    <p>Refresh Token: {{ userStore.currentUser.refresh_token }}</p>
  </div>
  <div v-else>
    <p>Not signed in</p>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/userStore.ts";

const userStore = useUserStore();

const callBackend = async () => {
  const port = import.meta.env.VITE_API_PORT;
  const response = await fetch(`https://localhost:${port}/api`, {
    method: "GET",
    headers: {
      "Authorization": "Bearer " + userStore.currentUser!.id_token,
    },
  });
  const message = await response.text();
  alert("message: " + message + "\n" + "status: " + response.status);
};
</script>
