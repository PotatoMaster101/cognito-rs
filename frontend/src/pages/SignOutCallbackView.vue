<template>
  <div>Signing out...</div>
</template>

<script setup lang="ts">
import { useUserStore } from "@/stores/userStore.ts";
import { onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const userStore = useUserStore();

onMounted(async () => {
  userStore.clearUser();
  await router.push("/");
});
</script>
