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

// レイアウト設定の定義
const layoutSettings = [
  { key: 'WindowSeparatorSize', label: 'ウィンドウ間のサイズ', description: 'ウィンドウ間のサイズ（ピクセル）', unit: 'px', min: 1, max: 50 },
  { key: 'ScrollBarSize', label: 'スクロールバーのサイズ', description: 'スクロールバーのサイズ（ピクセル）', unit: 'px', min: 10, max: 50 },
  { key: 'FooterHeight', label: 'フッターの高さ', description: 'フッターの高さ（ピクセル）', unit: 'px', min: 20, max: 100 },
  { key: 'TitleHeaderHeight', label: 'タイトルヘッダーのサイズ', description: 'タイトルヘッダーのサイズ（ピクセル）', unit: 'px', min: 10, max: 50 },
  { key: 'TimeGaugeHeight', label: 'タイムゲージの高さ', description: 'タイムゲージの高さ（ピクセル）', unit: 'px', min: 20, max: 100 },
  { key: 'LayerHeight', label: 'レイヤーの高さ', description: 'レイヤーの高さ（レイヤー編集）（ピクセル）', unit: 'px', min: 20, max: 100 },
  { key: 'LayerHeaderWidth', label: 'レイヤー名の幅', description: 'レイヤー名の幅（レイヤー編集）（ピクセル）', unit: 'px', min: 50, max: 300 },
  { key: 'SettingItemHeaderWidth', label: '設定項目ヘッダー幅', description: '設定項目のヘッダー幅（ピクセル）', unit: 'px', min: 50, max: 300 },
  { key: 'SettingItemHeight', label: '設定項目の高さ', description: '設定項目の高さ（ピクセル）', unit: 'px', min: 15, max: 50 },
  { key: 'SettingItemMarginWidth', label: '設定項目マージン幅', description: '設定項目のマージン幅（ピクセル）', unit: 'px', min: 0, max: 50 },
  { key: 'SettingHeaderHeight', label: '設定ヘッダーの高さ', description: '設定のヘッダーの高さ（ピクセル）', unit: 'px', min: 30, max: 100 },
  { key: 'PlayerControlHeight', label: 'プレイヤーコントローラーの高さ', description: 'プレイヤーコントローラーの高さ（ピクセル）', unit: 'px', min: 30, max: 100 },
  { key: 'ExplorerHeaderHeight', label: 'メディアエクスプローラーヘッダーの高さ', description: 'メディアエクスプローラーヘッダーの高さ（ピクセル）', unit: 'px', min: 20, max: 100 },
  { key: 'ExplorerWindowNum', label: 'メディアエクスプローラーの数', description: 'メディアエクスプローラーの数', unit: '', min: 1, max: 10 },
  { key: 'ListItemHeight', label: 'リスト選択項目の高さ', description: 'リスト選択項目の高さ（ピクセル）', unit: 'px', min: 15, max: 50 }
];

function updateValue(key: string, value: string) {
  const newValue = { ...props.modelValue };
  newValue[key] = value;
  emit('update:modelValue', newValue);
}

function getCurrentValue(key: string): string {
  return props.modelValue[key] || '';
}

function validateNumber(value: string, min: number, max: number): boolean {
  const num = parseInt(value);
  return !isNaN(num) && num >= min && num <= max;
}

function resetItem(key: string) {
  emit('update:modelValue', { ...props.modelValue, [key]: props.defaultValue[key] || '' });
}
</script>

<template>
  <div class="layout-section">
    <h2>レイアウト設定</h2>
    <p class="section-description">
      AviUtl2の各要素のサイズやレイアウトを設定できます。
    </p>

    <div class="settings-grid">
      <div 
        v-for="setting in layoutSettings" 
        :key="setting.key"
        class="setting-item"
      >
        <div class="setting-header">
          <label :for="setting.key" class="setting-label">{{ setting.label }}</label>
          <span class="setting-description">{{ setting.description }}</span>
          <button class="reset-btn" @click="resetItem(setting.key)">リセット</button>
        </div>
        
        <div class="setting-controls">
          <!-- 数値入力 -->
          <div class="input-group">
            <label :for="setting.key" class="input-label">値</label>
            <div class="input-with-unit">
              <input
                :id="setting.key"
                type="number"
                :value="getCurrentValue(setting.key)"
                @input="(e) => updateValue(setting.key, (e.target as HTMLInputElement).value)"
                :min="setting.min"
                :max="setting.max"
                class="number-input"
                :placeholder="setting.min.toString()"
              />
              <span class="unit-label">{{ setting.unit }}</span>
            </div>
          </div>

          <!-- スライダー -->
          <div class="input-group">
            <label class="input-label">スライダー</label>
            <input
              type="range"
              :value="getCurrentValue(setting.key)"
              @input="(e) => updateValue(setting.key, (e.target as HTMLInputElement).value)"
              :min="setting.min"
              :max="setting.max"
              class="range-slider"
            />
          </div>

          <!-- プレビュー -->
          <div class="preview-group">
            <label class="input-label">プレビュー</label>
            <div class="layout-preview">
              <div 
                class="preview-element"
                :style="{
                  width: setting.key.includes('Width') ? (getCurrentValue(setting.key) || setting.min) + 'px' : 'auto',
                  height: setting.key.includes('Height') || setting.key.includes('Size') ? (getCurrentValue(setting.key) || setting.min) + 'px' : 'auto',
                  minWidth: setting.key.includes('Width') ? (getCurrentValue(setting.key) || setting.min) + 'px' : 'auto',
                  minHeight: setting.key.includes('Height') || setting.key.includes('Size') ? (getCurrentValue(setting.key) || setting.min) + 'px' : 'auto'
                }"
              >
                <span class="preview-text">{{ setting.label }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- バリデーションエラー -->
        <div 
          v-if="getCurrentValue(setting.key) && !validateNumber(getCurrentValue(setting.key), setting.min, setting.max)"
          class="validation-error"
        >
          値は {{ setting.min }} から {{ setting.max }} の間で入力してください
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.layout-section {
  padding: 20px;
}

.layout-section h2 {
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
  grid-template-columns: 1fr 1fr 1fr;
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

.input-with-unit {
  display: flex;
  align-items: center;
  border: 1px solid #ddd;
  border-radius: 4px;
  background-color: white;
  overflow: hidden;
}

.number-input {
  flex: 1;
  padding: 8px 12px;
  border: none;
  font-size: 14px;
  background: none;
}

.number-input:focus {
  outline: none;
}

.unit-label {
  padding: 8px 12px;
  background-color: #f8f9fa;
  color: #666;
  font-size: 12px;
  font-weight: 500;
  border-left: 1px solid #ddd;
}

.range-slider {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: #ddd;
  outline: none;
  -webkit-appearance: none;
}

.range-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #007bff;
  cursor: pointer;
}

.range-slider::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #007bff;
  cursor: pointer;
  border: none;
}

.preview-group {
  display: flex;
  flex-direction: column;
}

.layout-preview {
  padding: 8px 12px;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  min-height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-element {
  background-color: #007bff;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 2px;
  padding: 2px 6px;
  font-size: 10px;
  font-weight: 500;
  text-align: center;
  line-height: 1.2;
  word-break: break-word;
}

.preview-text {
  font-size: 10px;
  line-height: 1.2;
}

.validation-error {
  margin-top: 10px;
  padding: 8px 12px;
  background-color: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
  border-radius: 4px;
  font-size: 12px;
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