<script setup lang="ts">
// import { ref } from 'vue';

interface Props {
  modelValue: Record<string, string>;
  defaultValue: Record<string, string>;
}

interface Emits {
  (e: 'update:modelValue', value: Record<string, string>): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// フォーマット設定の定義
const formatSettings = [
  { 
    key: 'FooterLeft', 
    label: 'フッター左側', 
    description: 'フッターの左側に表示される情報',
    placeholder: '{CurrentTime} / {TotalTime}  |  {CurrentFrame} / {TotalFrame}'
  },
  { 
    key: 'FooterRight', 
    label: 'フッター右側', 
    description: 'フッターの右側に表示される情報',
    placeholder: '{SceneName}  |  {Resolution}  |  {FrameRate}  |  {SamplingRate}'
  }
];

// 利用可能な変数
const availableVariables = [
  { variable: '{ProjectName}', description: 'プロジェクト名' },
  { variable: '{SceneName}', description: 'シーン名' },
  { variable: '{Resolution}', description: 'シーンの解像度' },
  { variable: '{FrameRate}', description: 'シーンのフレームレート' },
  { variable: '{SamplingRate}', description: 'シーンのサンプリングレート' },
  { variable: '{CurrentTime}', description: '現在の時間' },
  { variable: '{TotalTime}', description: '総時間' },
  { variable: '{CurrentFrame}', description: '現在のフレーム番号' },
  { variable: '{TotalFrame}', description: '総フレーム数' }
];

function updateValue(key: string, value: string) {
  const newValue = { ...props.modelValue };
  newValue[key] = value;
  emit('update:modelValue', newValue);
}

function insertVariable(key: string, variable: string) {
  const currentValue = props.modelValue[key] || '';
  updateValue(key, currentValue + variable);
}

function getCurrentValue(key: string): string {
  return props.modelValue[key] || '';
}

function resetItem(key: string) {
  emit('update:modelValue', { ...props.modelValue, [key]: props.defaultValue[key] || '' });
}
</script>

<template>
  <div class="format-section">
    <h2>フォーマット設定</h2>
    <p class="section-description">
      フッターに表示される情報のフォーマットを設定できます。変数を組み合わせてカスタマイズできます。
    </p>

    <div class="variables-section">
      <h3>利用可能な変数</h3>
      <div class="variables-grid">
        <button
          v-for="varItem in availableVariables"
          :key="varItem.variable"
          @click="() => insertVariable('FooterLeft', varItem.variable)"
          class="variable-button"
          :title="varItem.description"
        >
          <span class="variable-name">{{ varItem.variable }}</span>
          <span class="variable-description">{{ varItem.description }}</span>
        </button>
      </div>
    </div>

    <div class="settings-grid">
      <div 
        v-for="setting in formatSettings" 
        :key="setting.key"
        class="setting-item"
      >
        <div class="setting-header">
          <label :for="setting.key" class="setting-label">{{ setting.label }}</label>
          <span class="setting-description">{{ setting.description }}</span>
          <button class="reset-btn" @click="resetItem(setting.key)">リセット</button>
        </div>
        
        <div class="setting-controls">
          <!-- テキストエリア -->
          <div class="input-group full-width">
            <label :for="setting.key" class="input-label">フォーマット</label>
            <textarea
              :id="setting.key"
              :value="getCurrentValue(setting.key)"
              @input="(e) => updateValue(setting.key, (e.target as HTMLTextAreaElement).value)"
              class="format-textarea"
              :placeholder="setting.placeholder"
              rows="3"
            ></textarea>
          </div>

          <!-- プレビュー -->
          <div class="preview-group full-width">
            <label class="input-label">プレビュー</label>
            <div class="format-preview">
              <div class="preview-content">
                {{ getCurrentValue(setting.key) || setting.placeholder }}
              </div>
            </div>
          </div>
        </div>

        <!-- クイック挿入ボタン -->
        <div class="quick-insert">
          <label class="input-label">クイック挿入</label>
          <div class="quick-buttons">
            <button
              v-for="varItem in availableVariables"
              :key="varItem.variable"
              @click="() => insertVariable(setting.key, varItem.variable)"
              class="quick-button"
              :title="varItem.description"
            >
              {{ varItem.variable }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.format-section {
  padding: 20px;
}

.format-section h2 {
  margin: 0 0 10px 0;
  color: #333;
  font-size: 1.5rem;
}

.section-description {
  color: #666;
  margin-bottom: 30px;
  line-height: 1.5;
}

.variables-section {
  margin-bottom: 30px;
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #e9ecef;
}

.variables-section h3 {
  margin: 0 0 15px 0;
  color: #333;
  font-size: 1.2rem;
}

.variables-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 10px;
}

.variable-button {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 10px;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.variable-button:hover {
  background-color: #e9ecef;
  border-color: #007bff;
}

.variable-name {
  font-family: 'Consolas', monospace;
  font-size: 12px;
  color: #007bff;
  font-weight: 600;
  margin-bottom: 2px;
}

.variable-description {
  font-size: 11px;
  color: #666;
  line-height: 1.2;
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
  gap: 15px;
}

.input-group {
  display: flex;
  flex-direction: column;
}

.input-group.full-width {
  grid-column: 1 / -1;
}

.input-label {
  font-size: 12px;
  font-weight: 500;
  color: #555;
  margin-bottom: 5px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.format-textarea {
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  background-color: white;
  font-family: 'Consolas', monospace;
  resize: vertical;
  min-height: 80px;
}

.format-textarea:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.preview-group {
  display: flex;
  flex-direction: column;
}

.preview-group.full-width {
  grid-column: 1 / -1;
}

.format-preview {
  padding: 12px;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  min-height: 60px;
  display: flex;
  align-items: center;
}

.preview-content {
  font-family: 'Consolas', monospace;
  font-size: 14px;
  color: #333;
  line-height: 1.4;
  word-break: break-word;
}

.quick-insert {
  margin-top: 15px;
  padding-top: 15px;
  border-top: 1px solid #e9ecef;
}

.quick-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 8px;
}

.quick-button {
  padding: 6px 12px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 11px;
  font-family: 'Consolas', monospace;
  cursor: pointer;
  transition: background-color 0.2s;
}

.quick-button:hover {
  background-color: #0056b3;
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
  .variables-grid {
    grid-template-columns: 1fr;
  }
  
  .quick-buttons {
    flex-direction: column;
  }
  
  .quick-button {
    text-align: center;
  }
}
</style> 