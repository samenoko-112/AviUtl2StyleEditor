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

// 色設定の定義
const colorSettings = [
  { key: 'Background', label: 'デフォルト背景色', description: 'デフォルトの背景色' },
  { key: 'WindowBorder', label: 'ウィンドウ枠線色', description: 'ウィンドウの枠線色' },
  { key: 'WindowSeparator', label: 'ウィンドウ間背景色', description: 'ウィンドウ間の背景色' },
  { key: 'Footer', label: 'フッター背景色', description: 'フッターの背景色' },
  { key: 'FooterProgress', label: 'フッター進捗色', description: 'フッターの進捗色' },
  { key: 'Grouping', label: 'グループ化背景色', description: 'グループ化の背景色' },
  { key: 'GroupingHover', label: 'グループ化ホバー色', description: 'グループ化のホバー色' },
  { key: 'GroupingSelect', label: 'グループ化選択色', description: 'グループ化の選択色' },
  { key: 'TitleHeader', label: 'タイトルヘッダー背景色', description: 'タイトルヘッダーの背景色' },
  { key: 'BorderSelect', label: '選択枠線色', description: '選択時の枠線色' },
  { key: 'Border', label: '枠線色', description: '通常の枠線色' },
  { key: 'BorderFocus', label: 'フォーカス枠線色', description: 'フォーカス時の枠線色' },
  { key: 'Text', label: 'テキスト色', description: '通常のテキスト色' },
  { key: 'TextDisable', label: '無効テキスト色', description: '無効時のテキスト色' },
  { key: 'TextSelect', label: '選択テキスト色', description: '選択時のテキスト色' },
  { key: 'ButtonBody', label: 'ボタン背景色', description: 'ボタンの背景色' },
  { key: 'ButtonBodyHover', label: 'ボタンホバー色', description: 'ボタンのホバー色' },
  { key: 'ButtonBodyPress', label: 'ボタンプレス色', description: 'ボタンのプレス色' },
  { key: 'ButtonBodyDisable', label: 'ボタン無効色', description: 'ボタンの無効色' },
  { key: 'ButtonBodySelect', label: 'ボタン選択色', description: 'ボタンの選択色' },
  { key: 'SliderCursor', label: 'スライダーカーソル色', description: 'スライダーカーソルの色' },
  { key: 'TrackBarRange', label: 'トラックバー範囲色', description: 'トラックバーの範囲色' },
  { key: 'ZoomGauge', label: 'ズームゲージ色', description: 'ズームゲージの色' },
  { key: 'ZoomGaugeHover', label: 'ズームゲージホバー色', description: 'ズームゲージのホバー色' },
  { key: 'ZoomGaugeOff', label: 'ズームゲージ無効色', description: 'ズームゲージの無効色' },
  { key: 'ZoomGaugeOffHover', label: 'ズームゲージ無効ホバー色', description: 'ズームゲージの無効ホバー色' },
  { key: 'FrameCursor', label: 'フレームカーソル色', description: 'フレームカーソルの色' },
  { key: 'FrameCursorWide', label: 'フレームカーソル幅広色', description: 'フレームカーソルの幅広色' },
  { key: 'PlayerCursor', label: 'プレイヤーカーソル色', description: 'プレイヤーカーソルの色' },
  { key: 'GuideLine', label: 'ガイドライン色', description: 'ガイドラインの色' },
  { key: 'Layer', label: 'レイヤー背景色', description: 'レイヤーの背景色' },
  { key: 'LayerHeader', label: 'レイヤーヘッダー背景色', description: 'レイヤーのヘッダー背景色' },
  { key: 'LayerHover', label: 'レイヤーホバー色', description: 'レイヤーのホバー色' },
  { key: 'LayerDisable', label: 'レイヤー無効色', description: 'レイヤーの無効色' },
  { key: 'LayerRange', label: 'レイヤー範囲背景色', description: 'レイヤー範囲の背景色' },
  { key: 'LayerRangeFrame', label: 'レイヤー範囲枠線色', description: 'レイヤー範囲の枠線色' },
  { key: 'ObjectVideo', label: '映像オブジェクト色', description: '映像オブジェクトの色' },
  { key: 'ObjectVideoSelect', label: '映像オブジェクト選択色', description: '映像オブジェクトの選択色' },
  { key: 'ObjectAudio', label: '音声オブジェクト色', description: '音声オブジェクトの色' },
  { key: 'ObjectAudioSelect', label: '音声オブジェクト選択色', description: '音声オブジェクトの選択色' },
  { key: 'ObjectControl', label: '制御オブジェクト色', description: '制御オブジェクトの色' },
  { key: 'ObjectControlSelect', label: '制御オブジェクト選択色', description: '制御オブジェクトの選択色' },
  { key: 'ObjectVideoFilter', label: '映像フィルタオブジェクト色', description: '映像フィルタオブジェクトの色' },
  { key: 'ObjectVideoFilterSelect', label: '映像フィルタオブジェクト選択色', description: '映像フィルタオブジェクトの選択色' },
  { key: 'ObjectAudioFilter', label: '音声フィルタオブジェクト色', description: '音声フィルタオブジェクトの色' },
  { key: 'ObjectAudioFilterSelect', label: '音声フィルタオブジェクト選択色', description: '音声フィルタオブジェクトの選択色' },
  { key: 'ObjectHover', label: 'オブジェクトホバー色', description: 'オブジェクトのホバー色' },
  { key: 'ObjectFocus', label: 'オブジェクトフォーカス色', description: 'オブジェクトのフォーカス色' },
  { key: 'ObjectSection', label: 'オブジェクト中間点色', description: 'オブジェクト中間点の色' },
  { key: 'ClippingObject', label: 'クリッピングオブジェクト色', description: 'クリッピングオブジェクトの色' },
  { key: 'ClippingObjectMask', label: 'クリッピングオブジェクトマスク色', description: 'クリッピングオブジェクトのマスク色' },
  { key: 'Anchor', label: 'アンカー枠色', description: 'アンカーの枠色' },
  { key: 'AnchorLine', label: 'アンカー線色', description: 'アンカーの線色' },
  { key: 'AnchorIn', label: 'アンカー開始色', description: 'アンカーの開始色' },
  { key: 'AnchorOut', label: 'アンカー終了色', description: 'アンカーの終了色' },
  { key: 'AnchorHover', label: 'アンカーホバー色', description: 'アンカーのホバー色' },
  { key: 'AnchorSelect', label: 'アンカー選択色', description: 'アンカーの選択色' },
  { key: 'AnchorEdge', label: 'アンカー縁色', description: 'アンカーの縁色' },
  { key: 'CenterGroup', label: '中心点色（グループ）', description: '中心点の色（グループ）' },
  { key: 'HandleX', label: 'ハンドルX軸色', description: 'X軸ハンドルの色' },
  { key: 'HandleY', label: 'ハンドルY軸色', description: 'Y軸ハンドルの色' },
  { key: 'HandleZ', label: 'ハンドルZ軸色', description: 'Z軸ハンドルの色' },
  { key: 'HandleXHover', label: 'ハンドルX軸ホバー色', description: 'X軸ハンドルのホバー色' },
  { key: 'HandleYHover', label: 'ハンドルY軸ホバー色', description: 'Y軸ハンドルのホバー色' },
  { key: 'HandleZHover', label: 'ハンドルZ軸ホバー色', description: 'Z軸ハンドルのホバー色' },
  { key: 'OutsideDisplay', label: '表示領域外色', description: '表示領域外の色' }
];

function updateValue(key: string, value: string) {
  const newValue = { ...props.modelValue };
  newValue[key] = value;
  emit('update:modelValue', newValue);
}

function splitColors(value: string): [string, string?] {
  const parts = value.split(',');
  return [parts[0] || '', parts[1]];
}

function updateGradientValue(key: string, idx: number, color: string) {
  const current = props.modelValue[key] || '';
  const [c1, c2] = splitColors(current);
  let newValue = '';
  if (idx === 0) {
    newValue = c2 !== undefined ? `${color},${c2}` : color;
  } else {
    newValue = `${c1 || '000000'},${color}`;
  }
  updateValue(key, newValue);
}

function isGradient(value: string): boolean {
  return value.includes(",");
}

function getGradientPreview(color: string): string {
  const [c1, c2] = splitColors(color);
  if (c2) {
    return `linear-gradient(90deg, #${c1}, #${c2})`;
  }
  return `#${c1}`;
}

function resetItem(key: string) {
  emit('update:modelValue', { ...props.modelValue, [key]: props.defaultValue[key] || '' });
}
</script>

<template>
  <div class="color-section">
    <h2>色設定</h2>
    <p class="section-description">
      AviUtl2の各要素で使用される色を設定できます。16進数形式（例: ff0000）で入力してください。
    </p>

    <div class="settings-grid">
      <div 
        v-for="setting in colorSettings" 
        :key="setting.key"
        class="setting-item"
      >
        <div class="setting-header">
          <label :for="setting.key" class="setting-label">{{ setting.label }}</label>
          <span class="setting-description">{{ setting.description }}</span>
          <button class="reset-btn" @click="resetItem(setting.key)">リセット</button>
        </div>
        
        <div class="setting-controls">
          <!-- 色入力 -->
          <div class="input-group">
            <label :for="setting.key" class="input-label">16進数値</label>
            <input
              :id="setting.key"
              type="text"
              :value="props.modelValue[setting.key] || ''"
              @input="(e) => updateValue(setting.key, (e.target as HTMLInputElement).value)"
              class="color-input"
              placeholder="ff0000"
              maxlength="8"
              pattern="[0-9a-fA-F]{6,8}"
            />
          </div>

          <!-- カラーピッカー（グラデーション対応） -->
          <div class="input-group">
            <label :for="`${setting.key}-picker`" class="input-label">カラーピッカー</label>
            <template v-if="isGradient(props.modelValue[setting.key] || '')">
              <input
                :id="`${setting.key}-picker1`"
                type="color"
                :value="'#' + (splitColors(props.modelValue[setting.key] || '')[0] || '000000')"
                @input="(e) => updateGradientValue(setting.key, 0, (e.target as HTMLInputElement).value.substring(1))"
                class="color-picker"
              />
              <input
                :id="`${setting.key}-picker2`"
                type="color"
                :value="'#' + (splitColors(props.modelValue[setting.key] || '')[1] || '000000')"
                @input="(e) => updateGradientValue(setting.key, 1, (e.target as HTMLInputElement).value.substring(1))"
                class="color-picker"
              />
            </template>
            <template v-else>
              <input
                :id="`${setting.key}-picker`"
                type="color"
                :value="'#' + (props.modelValue[setting.key] || '000000')"
                @input="(e) => updateValue(setting.key, (e.target as HTMLInputElement).value.substring(1))"
                class="color-picker"
              />
            </template>
          </div>

          <!-- プレビュー -->
          <div class="preview-group">
            <label class="input-label">プレビュー</label>
            <div 
              class="color-preview"
              :style="{ background: getGradientPreview(props.modelValue[setting.key] || '000000') }"
            >
              <span class="preview-text">{{ setting.label }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.color-section {
  padding: 20px;
}

.color-section h2 {
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

.color-input {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  background-color: white;
  font-family: 'Consolas', monospace;
}

.color-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.color-picker {
  width: 100%;
  height: 40px;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  background: none;
}

.color-picker::-webkit-color-swatch-wrapper {
  padding: 0;
}

.color-picker::-webkit-color-swatch {
  border: none;
  border-radius: 4px;
}

.preview-group {
  display: flex;
  flex-direction: column;
}

.color-preview {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  min-height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  text-shadow: 1px 1px 1px rgba(0, 0, 0, 0.5);
  font-size: 12px;
  font-weight: 500;
}

.preview-text {
  text-align: center;
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