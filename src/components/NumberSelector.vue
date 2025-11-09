<script setup>
import { ref, watchEffect, onMounted } from "vue";

const props = defineProps({
  modelValue: {
    type: Number,
    default: null, // optional, if parent wants to control it
  },
  min: {
    type: Number,
    default: 1,
  },
  max: {
    type: Number,
    default: 8,
  },
  label: {
    type: String,
    default: "",
  },
  defaultValue: {
    type: Number,
    default: 3, // <-- your default
  },
});

const emit = defineEmits(["update:modelValue"]);
const localValue = ref(props.defaultValue);

// Sync localValue if parent explicitly sets v-model
watchEffect(() => {
  if (props.modelValue !== null && props.modelValue !== undefined) {
    localValue.value = props.modelValue;
  }
});

const update = (val) => {
  if (val >= props.min && val <= props.max) {
    localValue.value = val;
    emit("update:modelValue", val);
  }
};

// Optional: ensure modelValue is initialized on mount if parent doesn't provide
onMounted(() => {
  onMounted(() => {
    if (props.modelValue === null || props.modelValue === undefined) {
      emit("update:modelValue", localValue.value);
    } else {
      // Make localValue match modelValue from parent
      localValue.value = props.modelValue;
    }
  });
});
</script>

<template>
  <div class="flex items-center justify-center space-x-4">
    <!-- Label -->
    <span v-if="label" class="text-lg font-medium w-32 text-right">
      {{ label }}
    </span>

    <!-- Control -->
    <div class="flex items-center space-x-3">
      <button
        class="btn btn-circle btn-sm"
        :disabled="localValue <= min"
        @click="update(localValue - 1)"
      >
        ‹
      </button>

      <span class="text-xl font-bold w-10 text-center">
        {{ localValue }}
      </span>

      <button
        class="btn btn-circle btn-sm"
        :disabled="localValue >= max"
        @click="update(localValue + 1)"
      >
        ›
      </button>
    </div>
  </div>
</template>
