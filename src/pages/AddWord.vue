<template>
  <div class="mt-2">
    <form @submit.prevent="submit">
      <div class="grid grid-cols-8 gap-2">
        <label>
          <select class="py-2 px-4 rounded border border-blue-200 w-full" v-model="type" v-on:change="test">
            <option selected disabled value="">Word Type</option>
            <option v-for="(_, form) in availableForms" :value="form" :key="form">{{ form.slice(0, -4) }}</option>
          </select>
        </label>
        <component @save="save" :is="availableForms[type]"></component>
      </div>
      <div class="flex justify-center">
        <button type="submit" class="py-2 px-4 bg-blue-500 text-white rounded">Submit</button>
      </div>
    </form>
  </div>
</template>

<script setup>
import { ref } from "vue";
import NounForm from "../components/NounForm.vue";
import VerbForm from "../components/VerbForm.vue";
import PropositionForm from "../components/PropositionForm.vue";
import PhraseForm from "../components/PhraseForm.vue";
import MedicalPhrasesForm from "../components/MedicalPhrasesForm.vue";
import {invoke} from "@tauri-apps/api";

let type = ref("");
let formData = ref({});
const availableForms = {
  NounForm,
  VerbForm,
  PhraseForm,
  MedicalPhrasesForm
};

function save(data) {
  formData.value = { ...data };
}
function submit() {
  let eventName = ""
  let propName = ""
  if (type.value === "VerbForm") {
    eventName = "add_verb";
    propName= "newVerb";
  }else if (type.value === "NounForm") {
    eventName = "add_noun";
    propName = "newNoun";
  }else if (type.value === "PhraseForm") {
    eventName = "add_phrase";
    propName= "newPhrase";
  }else if (type.value === "MedicalPhrasesForm") {
    eventName = "add_medical_phrase";
    propName= "newMedicalPhrase";
  }
  invoke(eventName, {[propName]:formData.value}).then((response) => {
    console.log(response);
  });
}
</script>