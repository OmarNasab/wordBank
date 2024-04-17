<script setup>

import Model from "../components/Model.vue";
import {useRoute} from "vue-router";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api";

const verb_id= parseInt(useRoute().params.id.toString())
const verb=ref({})
const propositions=ref([])
const prefixes=ref([])
const propositionForm=ref({
  verb_id:verb_id,
  proposition:"",
  definition:""
})

const prefixForm=ref({
  verb_id:verb_id,
  prefix:"",
  definition:""
})

function submit_proposition(){
  invoke("add_proposition", {newProposition:propositionForm.value}).then((res)=>{
    propositions.value.push(res)
  })
}

function submit_prefix(){
  invoke("add_prefix", {newPrefix:prefixForm.value}).then((res)=>{
    prefixes.value.push(res)
  })
}


invoke("get_verb",{id:verb_id}).then((res)=>{
  verb.value=res
})

invoke("get_propositions",{verb:verb_id}).then((res)=>{
  propositions.value=res
})

invoke("get_prefixes",{verb:verb_id}).then((res)=>{
  prefixes.value=res
})


</script>

<template>
  <div class="flex justify-between">
    <h2 class="text-2xl font-semibold">{{verb.verb}}</h2>
    <div>
      <Model>
        <template #button_text>
          Add Proposition
        </template>
        <form @submit.prevent="submit_proposition">
          <input v-model="propositionForm.proposition" type="text" class="border rounded px-4 py-2 w-full" placeholder="proposition">
          <input v-model="propositionForm.definition" type="text" class="border rounded px-4 py-2 w-full" placeholder="definition">
          <button class="bg-blue-500 py-2 px-4">Save</button>
        </form>
      </Model>
      <Model>
        <template #button_text>
          Add Prefix
        </template>
        <form @submit.prevent="submit_prefix">
          <input v-model="prefixForm.prefix" type="text" class="border rounded px-4 py-2 w-full" placeholder="prefix">
          <input v-model="prefixForm.definition" type="text" class="border rounded px-4 py-2 w-full" placeholder="definition">
          <button class="bg-blue-500 py-2 px-4">Save</button>
        </form>
      </Model>
    </div>
  </div>
  <div>
    Prefixes
    <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
      <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
      <tr>
        <th scope="col" class="px-6 py-3">
          Prefix
        </th>
        <th scope="col" class="px-6 py-3">
          definition
        </th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="prefix in prefixes" class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
          {{prefix.prefix}}
        </th>
        <td class="px-6 py-4">
          {{prefix.definition}}
        </td>
      </tr>
      </tbody>
    </table>
  </div>
  <div>
    Propositions
    <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
      <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
      <tr>
        <th scope="col" class="px-6 py-3">
          Proposition
        </th>
        <th scope="col" class="px-6 py-3">
          definition
        </th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="proposition in propositions" class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
          {{proposition.proposition}}
        </th>
        <td class="px-6 py-4">
          {{proposition.definition}}
        </td>
      </tr>
      </tbody>
    </table>
  </div>

</template>

<style scoped>

</style>