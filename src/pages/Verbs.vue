<script setup>

import {ref} from "vue";
import {invoke} from "@tauri-apps/api";

const verbs=ref([])

invoke("get_verbs").then((res)=>{
  verbs.value=res
})

</script>

<template>


  <div class="mt-2">
    <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
      <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
      <tr>
        <th scope="col" class="px-6 py-3">
          Verb
        </th>
        <th scope="col" class="px-6 py-3">
          präteritum
        </th>
        <th scope="col" class="px-6 py-3">
          Perfekt
        </th>
        <th scope="col" class="px-6 py-3">
          definition
        </th>
        <th scope="col" class="px-6 py-3">
          action
        </th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="verb in verbs" class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
          {{verb.verb}}
        </th>
        <td class="px-6 py-4">
          {{verb.past_simple}}
        </td>
        <td class="px-6 py-4">
          {{verb.past_perfect}}
        </td>
        <td class="px-6 py-4">
          {{verb.definition}}
        </td>
        <td class="px-6 py-4">
          <router-link :to="'/verbs/'+verb.id">View</router-link>
        </td>
      </tr>
      </tbody>
    </table>
  </div>

</template>

<style scoped>

</style>