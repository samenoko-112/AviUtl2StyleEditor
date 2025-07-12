<script setup lang="ts">
import { computed, ref, reactive, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface Props {
  modelValue: Record<string, string>;
  fontFamilies: string[];
  defaultValue: Record<string, string>;
}

interface Emits {
  (e: 'update:modelValue', value: Record<string, string>): void;
}

interface FontSetting {
  key: string;
  label: string;
  description: string;
  input_type: string;
}

interface FontValue {
  size: string;
  family: string;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// フォント設定をバックエンドから取得
const fontSettings = ref<FontSetting[]>([]);
const searchQuery = ref('');
const fontFamilyModels = reactive<Record<string, string>>({});

onMounted(async () => {
  try {
    fontSettings.value = await invoke('get_font_settings');
    
    for (const setting of fontSettings.value) {
      const parsed = await parseFontValue(props.modelValue[setting.key] || '');
      fontFamilyModels[setting.key] = parsed.family;
    }
  } catch (error) {
    console.error('フォント設定の取得に失敗しました:', error);
  }
});

const filteredFontFamilies = computed(() => {
  if (!searchQuery.value) {
    return props.fontFamilies;
  }
  return props.fontFamilies.filter(font => 
    font.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

function updateValue(key: string, value: string) {
  const newValue = { ...props.modelValue };
  newValue[key] = value;
  emit('update:modelValue', newValue);
}

async function onFontFamilyChange(key: string) {
  const current = await parseFontValue(props.modelValue[key] || '');
  const formatted = await invoke('format_font_value', {
    size: current.size,
    family: fontFamilyModels[key],
    key: key
  });
  updateValue(key, formatted as string);
}

async function parseFontValue(value: string): Promise<FontValue> {
  try {
    return await invoke('parse_font_value', { value });
  } catch (error) {
    console.error('フォント値の解析に失敗しました:', error);
    return { size: '', family: '' };
  }
}

async function formatFontValue(size: string, family: string, key: string): Promise<string> {
  try {
    return await invoke('format_font_value', { size, family, key }) as string;
  } catch (error) {
    console.error('フォント値のフォーマットに失敗しました:', error);
    return size;
  }
}

function resetItem(key: string) {
  emit('update:modelValue', { ...props.modelValue, [key]: props.defaultValue[key] || '' });
}
</script>

<template>
  <div class="font-section">
    <h2>フォント設定</h2>
    <p class="section-description">
      AviUtl2の各要素で使用されるフォントとサイズを設定できます。
    </p>

    <div class="settings-grid">
      <div 
        v-for="setting in fontSettings" 
        :key="setting.key"
        class="setting-item"
      >
        <div class="setting-header">
          <label :for="setting.key" class="setting-label">{{ setting.label }}</label>
          <span class="setting-description">{{ setting.description }}</span>
          <button class="reset-btn" @click="resetItem(setting.key)">リセット</button>
        </div>
        
        <div class="setting-controls">
          <!-- フォントサイズ入力 -->
          <div class="input-group" v-if="setting.input_type !== 'familyOnly'">
            <label :for="`${setting.key}-size`" class="input-label">サイズ</label>
            <input
              :id="`${setting.key}-size`"
              type="number"
              :value="(async () => {
                const parsed = await parseFontValue(props.modelValue[setting.key] || '');
                return parsed.size;
              })()"
              @input="async (e) => {
                const current = await parseFontValue(props.modelValue[setting.key] || '');
                const formatted = await formatFontValue((e.target as HTMLInputElement).value, current.family, setting.key);
                updateValue(setting.key, formatted);
              }"
              min="8"
              max="72"
              class="size-input"
              placeholder="13"
            />
          </div>

          <!-- フォントファミリー選択 -->
          <div class="input-group" v-if="setting.input_type !== 'sizeOnly'">
            <label :for="`${setting.key}-family`" class="input-label">フォント</label>
            <input
              v-model="searchQuery"
              type="text"
              placeholder="フォントを検索..."
              class="font-search"
            />
            <select
              :id="`${setting.key}-family`"
              v-model="fontFamilyModels[setting.key]"
              @change="onFontFamilyChange(setting.key)"
              class="family-select"
            >
              <option value="">デフォルト</option>
              <option 
                v-for="family in filteredFontFamilies" 
                :key="family" 
                :value="family"
              >
                {{ family }}
              </option>
            </select>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.font-section {
  padding: 20px;
}

.font-section h2 {
  margin: 0 0 10px 0;
  color: #333;
  font-size: 1.5rem;
}

.section-description {
  color: #666;
  margin-bottom: 30px;
  line-height: 1.5;
}

.settings-grid {
  display: grid;
  gap: 20px;
}

.setting-item {
  background-color: #f8f9fa;
  border-radius: 8px;
  padding: 20px;
  border: 1px solid #e9ecef;
}

.setting-header {
  margin-bottom: 15px;
  display: flex;
  align-items: center;
}

.setting-label {
  display: block;
  font-weight: 600;
  color: #333;
  margin-bottom: 5px;
  font-size: 16px;
}

.setting-description {
  display: block;
  color: #666;
  font-size: 14px;
  line-height: 1.4;
  margin-left: 10px;
}

.setting-controls {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 15px;
  align-items: end;
}

.input-group {
  display: flex;
  flex-direction: column;
}

.input-label {
  font-size: 12px;
  font-weight: 500;
  color: #555;
  margin-bottom: 5px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.size-input {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  background-color: white;
}

.size-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.family-select {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  background-color: white;
  cursor: pointer;
}

.family-select:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.font-search {
  padding: 6px 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 12px;
  background-color: white;
  margin-bottom: 5px;
}

.font-search:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.preview-group {
  display: flex;
  flex-direction: column;
}

.font-preview {
  padding: 8px 12px;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  min-height: 20px;
  display: flex;
  align-items: center;
  color: #333;
  word-break: break-word;
  line-height: 1.2;
}

.reset-btn {
  margin-left: 12px;
  padding: 2px 10px;
  font-size: 12px;
  background: #eee;
  border: 1px solid #bbb;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}
.reset-btn:hover {
  background: #ffd;
  border-color: #e0c200;
}

@media (max-width: 768px) {
  .setting-controls {
    grid-template-columns: 1fr;
    gap: 10px;
  }
}
</style> 