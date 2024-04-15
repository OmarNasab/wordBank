

<template>
  <div class="my-24 mx-16 p-4 bg-white rounded-lg shadow-xl min-h-96">
    <div class="flex gap-2 justify-end items-center">
      <div class="grow flex justify-end">
        <input v-model="search" class="px-4 py-2 rounded-lg bg-blue-300 w-3/4" type="text">
        <button v-on:click="checkTauri">search</button>
      </div>
      <div>
        <router-link class="bg-blue-500 px-4 py-2 text-white rounded" to="/add-word">Add Word</router-link>
      </div>
    </div>
    <div class="mt-2">
      <navbar></navbar>
      <router-view></router-view>
    </div>

  </div>

</template>
<script setup>

import Navbar from "../components/Navbar.vue";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api";

let search=ref("")
function checkTauri(){
  invoke('greet', { name: search.value })
      // `invoke` returns a Promise
      .then((response) => console.log(response))
}
</script>

