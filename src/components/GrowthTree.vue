<script setup lang="ts">
import { computed, ref } from 'vue';
import type { TreeStageInfo } from '../types/bindings/TreeStageInfo';
import { Sparkles, Info, EyeOff } from 'lucide-vue-next';

interface Props {
  treeStage: TreeStageInfo;
  accuracy?: number;
  streak?: number;
}

const props = withDefaults(defineProps<Props>(), {
  accuracy: 100,
  streak: 0,
});

// Allow user to preview all stages if they want to see what's ahead
const previewStage = ref<number | null>(null);

const activeStage = computed(() => previewStage.value ?? props.treeStage.stage);

const stageNames = [
  'Seedling Sprout',
  'Young Sapling',
  'Branching Bonsai',
  'Verdant Canopy',
  'Master Full Bloom',
];

const stageTitles = computed(() => {
  if (previewStage.value !== null) {
    return {
      name: stageNames[previewStage.value - 1] || props.treeStage.stageName,
      desc: getStageDescription(previewStage.value),
    };
  }
  return {
    name: props.treeStage.stageName,
    desc: props.treeStage.stageDescription,
  };
});

function getStageDescription(stage: number): string {
  switch (stage) {
    case 1:
      return 'A tender shoot taking root in the soil. Practice consistently to help it grow sturdy.';
    case 2:
      return 'A resilient young sapling developing its main stem and first lateral branches.';
    case 3:
      return 'A defined bonsai with balanced branches. Keystroke rhythm and accuracy are taking shape.';
    case 4:
      return 'A flourishing canopy with dense foliage. Speed and precision are working in harmony.';
    case 5:
      return 'A majestic bonsai in full floral blossom. Fluid typing and daily discipline achieved.';
    default:
      return '';
  }
}

// Leaf cloud opacity and scale based on foliage density (accuracy)
const foliageOpacity = computed(() => {
  const density = props.treeStage.foliageDensity ?? 0.8;
  return Math.min(1.0, Math.max(0.4, density));
});

// Flower petal count tied to streak
const displayedBlooms = computed(() => {
  if (activeStage.value < 5) return 0;
  return Math.max(3, Math.min(12, props.treeStage.bloomCount || 4));
});
</script>

<template>
  <div class="relative flex flex-col justify-between overflow-hidden rounded-2xl border border-border bg-card p-6 shadow-sm">
    <!-- Header: Level & Stage Information -->
    <div class="flex items-start justify-between gap-4">
      <div>
        <div class="flex items-center gap-2">
          <span class="inline-flex items-center gap-1.5 rounded-full border border-primary/30 bg-primary/10 px-2.5 py-0.5 text-xs font-semibold text-primary-foreground">
            <Sparkles class="h-3 w-3" />
            Stage {{ activeStage }} of 5
          </span>
          <span v-if="previewStage !== null" class="rounded-full bg-amber-500/10 border border-amber-500/20 px-2 py-0.5 text-[10px] font-medium text-amber-500">
            Preview Mode
          </span>
        </div>
        <h3 class="mt-2 text-2xl font-bold font-display tracking-tight text-foreground">
          {{ stageTitles.name }}
        </h3>
        <p class="mt-1 text-xs text-muted-foreground max-w-sm line-clamp-2">
          {{ stageTitles.desc }}
        </p>
      </div>

      <!-- Preview controls -->
      <div class="flex items-center gap-1">
        <button
          v-if="previewStage !== null"
          type="button"
          class="inline-flex items-center gap-1 rounded-lg border border-border bg-surface px-2 py-1 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors"
          title="Return to your actual tree stage"
          @click="previewStage = null"
        >
          <EyeOff class="h-3.5 w-3.5" />
          <span>Reset</span>
        </button>
        <div class="flex items-center rounded-lg border border-border bg-surface/50 p-0.5 text-xs">
          <button
            v-for="s in 5"
            :key="s"
            type="button"
            class="h-6 w-6 rounded-lg text-[11px] font-medium transition-all"
            :class="[
              activeStage === s
                ? 'bg-primary text-primary-foreground shadow-sm font-semibold'
                : 'bg-surface/80 text-muted-foreground/70 hover:text-foreground hover:bg-surface'
            ]"
            :title="`Preview Stage ${s}: ${stageNames[s - 1]}`"
            @click="previewStage = s === props.treeStage.stage ? null : s"
          >
            {{ s }}
          </button>
        </div>
      </div>
    </div>

    <!-- Tree SVG Illustration -->
    <div class="relative my-3 flex h-60 w-full items-center justify-center select-none">
      <svg
        viewBox="0 0 400 320"
        class="h-full w-full max-w-md transition-all duration-700 ease-out"
        role="img"
        aria-label="Progress Growth Tree"
      >
        <defs>
          <!-- Gradients for subtle depth -->
          <linearGradient id="trunkGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#78593a" />
            <stop offset="60%" stop-color="#543c24" />
            <stop offset="100%" stop-color="#3b2917" />
          </linearGradient>

          <linearGradient id="leafGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#34d399" />
            <stop offset="100%" stop-color="#059669" />
          </linearGradient>

          <linearGradient id="leafGradDark" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#10b981" />
            <stop offset="100%" stop-color="#047857" />
          </linearGradient>

          <linearGradient id="bloomGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#fbcfe8" />
            <stop offset="60%" stop-color="#f472b6" />
            <stop offset="100%" stop-color="#db2777" />
          </linearGradient>

          <radialGradient id="soilDishGrad" cx="50%" cy="40%" r="50%">
            <stop offset="0%" stop-color="#334155" stop-opacity="0.3" />
            <stop offset="100%" stop-color="#0f172a" stop-opacity="0.8" />
          </radialGradient>

          <radialGradient id="zenAura" cx="50%" cy="45%" r="48%">
            <stop offset="0%" stop-color="var(--primary)" stop-opacity="0.12" />
            <stop offset="70%" stop-color="var(--primary)" stop-opacity="0.03" />
            <stop offset="100%" stop-color="var(--primary)" stop-opacity="0" />
          </radialGradient>

          <filter id="softGlow" x="-20%" y="-20%" width="140%" height="140%">
            <feGaussianBlur stdDeviation="3" result="blur" />
            <feComposite in="SourceGraphic" in2="blur" operator="over" />
          </filter>
        </defs>

        <!-- Ambient circular aura -->
        <circle cx="200" cy="160" r="130" fill="url(#zenAura)" />

        <!-- Zen Ceramic Pot / Flat Slab base -->
        <g class="transition-transform duration-500">
          <ellipse cx="200" cy="275" rx="88" ry="12" fill="url(#soilDishGrad)" />
          <path
            d="M 125 272 C 125 284, 275 284, 275 272 C 275 268, 125 268, 125 272 Z"
            fill="#1e293b"
            stroke="#475569"
            stroke-width="1.5"
          />
          <ellipse cx="200" cy="270" rx="72" ry="7" fill="#3f2e1e" />
          <ellipse cx="200" cy="269" rx="68" ry="5.5" fill="#291c10" />
          <!-- Small zen moss patches -->
          <circle cx="160" cy="269" r="4.5" fill="#15803d" opacity="0.8" />
          <circle cx="235" cy="270" r="5" fill="#16a34a" opacity="0.75" />
          <circle cx="180" cy="271" r="3.5" fill="#15803d" opacity="0.85" />
        </g>

        <!-- ================= STAGE 1: SPROUT ================= -->
        <g v-if="activeStage === 1" class="transition-all duration-700">
          <!-- Young tender stem -->
          <path
            d="M 200 268 C 198 245, 194 220, 201 195"
            fill="none"
            stroke="#10b981"
            stroke-width="4.5"
            stroke-linecap="round"
          />
          <!-- Left seed cotyledon leaf -->
          <path
            d="M 197 215 C 182 210, 168 216, 172 226 C 176 235, 194 226, 197 218 Z"
            fill="url(#leafGrad)"
            stroke="#059669"
            stroke-width="1.2"
          />
          <!-- Right fresh leaf -->
          <path
            d="M 201 200 C 218 190, 232 195, 230 206 C 228 216, 208 211, 201 202 Z"
            fill="url(#leafGrad)"
            stroke="#059669"
            stroke-width="1.2"
          />
          <!-- Sprout tip bud with glow -->
          <circle cx="201" cy="194" r="3.5" fill="#6ee7b7" filter="url(#softGlow)" />
          <!-- Morning dewdrop -->
          <circle cx="174" cy="223" r="1.5" fill="#ffffff" opacity="0.9" />
        </g>

        <!-- ================= STAGE 2: SAPLING ================= -->
        <g v-else-if="activeStage === 2" class="transition-all duration-700">
          <!-- Growing trunk -->
          <path
            d="M 200 268 C 196 235, 206 205, 198 165"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="6.5"
            stroke-linecap="round"
          />
          <!-- Lateral branches -->
          <path
            d="M 198 220 C 182 212, 168 214, 154 218"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="3.5"
            stroke-linecap="round"
          />
          <path
            d="M 200 195 C 215 186, 230 188, 244 193"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="3.5"
            stroke-linecap="round"
          />
          <!-- Foliage clusters -->
          <g :opacity="foliageOpacity">
            <!-- Left foliage -->
            <ellipse cx="150" cy="216" rx="16" ry="11" fill="url(#leafGrad)" />
            <ellipse cx="143" cy="213" rx="11" ry="8" fill="url(#leafGradDark)" />
            <!-- Right foliage -->
            <ellipse cx="248" cy="191" rx="17" ry="12" fill="url(#leafGrad)" />
            <ellipse cx="254" cy="189" rx="12" ry="9" fill="url(#leafGradDark)" />
            <!-- Top crown cluster -->
            <ellipse cx="197" cy="158" rx="22" ry="15" fill="url(#leafGrad)" />
            <ellipse cx="195" cy="152" rx="16" ry="11" fill="url(#leafGradDark)" />
          </g>
        </g>

        <!-- ================= STAGE 3: BRANCHING BONSAI ================= -->
        <g v-else-if="activeStage === 3" class="transition-all duration-700">
          <!-- Curving artistic bonsai trunk -->
          <path
            d="M 200 268 C 192 245, 182 225, 193 195 C 202 170, 195 150, 198 135"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="9"
            stroke-linecap="round"
          />
          <!-- Primary side branches -->
          <path
            d="M 188 225 C 165 218, 142 224, 126 230"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="5"
            stroke-linecap="round"
          />
          <path
            d="M 194 185 C 218 175, 245 180, 268 188"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="4.5"
            stroke-linecap="round"
          />
          <path
            d="M 197 150 C 180 142, 162 144, 148 149"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="3.5"
            stroke-linecap="round"
          />

          <!-- Cloud foliage pads -->
          <g :opacity="foliageOpacity" class="transition-opacity duration-500">
            <!-- Lower Left Cloud -->
            <g transform="translate(122, 226)">
              <ellipse cx="0" cy="0" rx="26" ry="14" fill="url(#leafGradDark)" />
              <ellipse cx="6" cy="-4" rx="22" ry="12" fill="url(#leafGrad)" />
              <ellipse cx="-8" cy="-2" rx="16" ry="10" fill="url(#leafGrad)" />
            </g>

            <!-- Middle Right Cloud -->
            <g transform="translate(268, 185)">
              <ellipse cx="0" cy="0" rx="30" ry="16" fill="url(#leafGradDark)" />
              <ellipse cx="-6" cy="-5" rx="24" ry="13" fill="url(#leafGrad)" />
              <ellipse cx="8" cy="-2" rx="18" ry="11" fill="url(#leafGrad)" />
            </g>

            <!-- Upper Left Cloud -->
            <g transform="translate(145, 146)">
              <ellipse cx="0" cy="0" rx="22" ry="13" fill="url(#leafGradDark)" />
              <ellipse cx="4" cy="-3" rx="18" ry="11" fill="url(#leafGrad)" />
            </g>

            <!-- Top Crown Cloud -->
            <g transform="translate(200, 128)">
              <ellipse cx="0" cy="0" rx="36" ry="20" fill="url(#leafGradDark)" />
              <ellipse cx="-8" cy="-6" rx="28" ry="16" fill="url(#leafGrad)" />
              <ellipse cx="10" cy="-4" rx="26" ry="15" fill="url(#leafGrad)" />
              <ellipse cx="0" cy="-10" rx="20" ry="12" fill="#6ee7b7" opacity="0.6" />
            </g>
          </g>
        </g>

        <!-- ================= STAGE 4: VERDANT CANOPY ================= -->
        <g v-else-if="activeStage === 4" class="transition-all duration-700">
          <!-- Robust old trunk with nebari surface roots -->
          <path
            d="M 188 270 Q 196 258 200 245 Q 204 258 212 270"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="5"
            stroke-linecap="round"
          />
          <path
            d="M 200 268 C 190 238, 178 218, 192 182 C 204 154, 192 134, 196 115"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="12"
            stroke-linecap="round"
          />
          <!-- Mature branch network -->
          <path d="M 186 218 C 158 210, 128 218, 106 226" fill="none" stroke="url(#trunkGrad)" stroke-width="6" stroke-linecap="round" />
          <path d="M 194 172 C 226 160, 260 168, 290 178" fill="none" stroke="url(#trunkGrad)" stroke-width="5.5" stroke-linecap="round" />
          <path d="M 193 140 C 168 130, 142 134, 124 140" fill="none" stroke="url(#trunkGrad)" stroke-width="4.5" stroke-linecap="round" />
          <path d="M 196 122 C 216 112, 238 116, 256 122" fill="none" stroke="url(#trunkGrad)" stroke-width="4" stroke-linecap="round" />

          <!-- Lush dense foliage clouds -->
          <g :opacity="foliageOpacity" class="transition-opacity duration-500">
            <!-- Low Tier Left -->
            <g transform="translate(102, 222)">
              <ellipse cx="0" cy="0" rx="34" ry="18" fill="url(#leafGradDark)" />
              <ellipse cx="8" cy="-5" rx="28" ry="15" fill="url(#leafGrad)" />
              <ellipse cx="-10" cy="-3" rx="22" ry="12" fill="url(#leafGrad)" />
            </g>

            <!-- Mid Tier Right -->
            <g transform="translate(292, 174)">
              <ellipse cx="0" cy="0" rx="38" ry="20" fill="url(#leafGradDark)" />
              <ellipse cx="-8" cy="-6" rx="30" ry="16" fill="url(#leafGrad)" />
              <ellipse cx="10" cy="-3" rx="24" ry="14" fill="url(#leafGrad)" />
            </g>

            <!-- Mid Tier Left -->
            <g transform="translate(120, 136)">
              <ellipse cx="0" cy="0" rx="32" ry="18" fill="url(#leafGradDark)" />
              <ellipse cx="6" cy="-4" rx="26" ry="14" fill="url(#leafGrad)" />
            </g>

            <!-- Upper Tier Right -->
            <g transform="translate(258, 118)">
              <ellipse cx="0" cy="0" rx="30" ry="16" fill="url(#leafGradDark)" />
              <ellipse cx="-6" cy="-4" rx="24" ry="13" fill="url(#leafGrad)" />
            </g>

            <!-- Majestic Top Canopy -->
            <g transform="translate(196, 105)">
              <ellipse cx="0" cy="0" rx="48" ry="26" fill="url(#leafGradDark)" />
              <ellipse cx="-14" cy="-8" rx="38" ry="22" fill="url(#leafGrad)" />
              <ellipse cx="14" cy="-6" rx="36" ry="20" fill="url(#leafGrad)" />
              <ellipse cx="0" cy="-14" rx="28" ry="16" fill="#34d399" opacity="0.75" />
            </g>
          </g>
        </g>

        <!-- ================= STAGE 5: FULL BLOOM ================= -->
        <g v-else class="transition-all duration-700">
          <!-- Master bonsai trunk -->
          <path
            d="M 186 270 Q 196 256 200 245 Q 204 256 214 270"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="6"
            stroke-linecap="round"
          />
          <path
            d="M 200 268 C 188 236, 176 216, 192 180 C 205 150, 192 130, 196 110"
            fill="none"
            stroke="url(#trunkGrad)"
            stroke-width="13"
            stroke-linecap="round"
          />
          <!-- Full branch spread -->
          <path d="M 186 216 C 156 208, 126 216, 100 224" fill="none" stroke="url(#trunkGrad)" stroke-width="6.5" stroke-linecap="round" />
          <path d="M 194 170 C 228 158, 264 166, 296 176" fill="none" stroke="url(#trunkGrad)" stroke-width="6" stroke-linecap="round" />
          <path d="M 193 138 C 166 128, 138 132, 118 138" fill="none" stroke="url(#trunkGrad)" stroke-width="5" stroke-linecap="round" />
          <path d="M 196 118 C 218 108, 242 112, 262 118" fill="none" stroke="url(#trunkGrad)" stroke-width="4.5" stroke-linecap="round" />

          <!-- Lush verdant canopy pads -->
          <g :opacity="foliageOpacity" class="transition-opacity duration-500">
            <!-- Lower Left -->
            <g transform="translate(98, 220)">
              <ellipse cx="0" cy="0" rx="36" ry="19" fill="url(#leafGradDark)" />
              <ellipse cx="8" cy="-5" rx="30" ry="16" fill="url(#leafGrad)" />
            </g>

            <!-- Mid Right -->
            <g transform="translate(298, 172)">
              <ellipse cx="0" cy="0" rx="42" ry="22" fill="url(#leafGradDark)" />
              <ellipse cx="-8" cy="-6" rx="34" ry="18" fill="url(#leafGrad)" />
            </g>

            <!-- Mid Left -->
            <g transform="translate(114, 134)">
              <ellipse cx="0" cy="0" rx="34" ry="19" fill="url(#leafGradDark)" />
              <ellipse cx="6" cy="-4" rx="28" ry="15" fill="url(#leafGrad)" />
            </g>

            <!-- Upper Right -->
            <g transform="translate(264, 114)">
              <ellipse cx="0" cy="0" rx="32" ry="17" fill="url(#leafGradDark)" />
              <ellipse cx="-6" cy="-4" rx="26" ry="14" fill="url(#leafGrad)" />
            </g>

            <!-- Crown -->
            <g transform="translate(196, 100)">
              <ellipse cx="0" cy="0" rx="52" ry="28" fill="url(#leafGradDark)" />
              <ellipse cx="-16" cy="-8" rx="42" ry="24" fill="url(#leafGrad)" />
              <ellipse cx="16" cy="-6" rx="40" ry="22" fill="url(#leafGrad)" />
              <ellipse cx="0" cy="-14" rx="30" ry="18" fill="#34d399" opacity="0.8" />
            </g>
          </g>

          <!-- ================= BLOSSOMS & PETALS ================= -->
          <g class="transition-opacity duration-700">
            <!-- Floral rosettes / blossoms -->
            <!-- Blossom 1: Crown peak -->
            <g transform="translate(196, 82) scale(0.95)" filter="url(#softGlow)">
              <circle cx="0" cy="0" r="6.5" fill="url(#bloomGrad)" />
              <circle cx="-5" cy="-3" r="4.5" fill="url(#bloomGrad)" />
              <circle cx="5" cy="-3" r="4.5" fill="url(#bloomGrad)" />
              <circle cx="-4" cy="4" r="4.5" fill="url(#bloomGrad)" />
              <circle cx="4" cy="4" r="4.5" fill="url(#bloomGrad)" />
              <circle cx="0" cy="0" r="2.5" fill="#fef08a" />
            </g>

            <!-- Blossom 2: Mid-right -->
            <g v-if="displayedBlooms >= 2" transform="translate(310, 160) scale(0.85)">
              <circle cx="0" cy="0" r="6" fill="url(#bloomGrad)" />
              <circle cx="-4" cy="-3" r="4" fill="url(#bloomGrad)" />
              <circle cx="4" cy="-3" r="4" fill="url(#bloomGrad)" />
              <circle cx="-3" cy="3" r="4" fill="url(#bloomGrad)" />
              <circle cx="3" cy="3" r="4" fill="url(#bloomGrad)" />
              <circle cx="0" cy="0" r="2" fill="#fef08a" />
            </g>

            <!-- Blossom 3: Lower-left -->
            <g v-if="displayedBlooms >= 3" transform="translate(86, 212) scale(0.85)">
              <circle cx="0" cy="0" r="6" fill="url(#bloomGrad)" />
              <circle cx="-4" cy="-3" r="4" fill="url(#bloomGrad)" />
              <circle cx="4" cy="-3" r="4" fill="url(#bloomGrad)" />
              <circle cx="-3" cy="3" r="4" fill="url(#bloomGrad)" />
              <circle cx="3" cy="3" r="4" fill="url(#bloomGrad)" />
              <circle cx="0" cy="0" r="2" fill="#fef08a" />
            </g>

            <!-- Blossom 4: Crown-left -->
            <g v-if="displayedBlooms >= 4" transform="translate(160, 96) scale(0.8)">
              <circle cx="0" cy="0" r="5" fill="url(#bloomGrad)" />
              <circle cx="-3" cy="-2" r="3.5" fill="url(#bloomGrad)" />
              <circle cx="3" cy="-2" r="3.5" fill="url(#bloomGrad)" />
              <circle cx="0" cy="0" r="1.8" fill="#fef08a" />
            </g>

            <!-- Blossom 5: Crown-right -->
            <g v-if="displayedBlooms >= 5" transform="translate(232, 94) scale(0.8)">
              <circle cx="0" cy="0" r="5" fill="url(#bloomGrad)" />
              <circle cx="-3" cy="-2" r="3.5" fill="url(#bloomGrad)" />
              <circle cx="3" cy="-2" r="3.5" fill="url(#bloomGrad)" />
              <circle cx="0" cy="0" r="1.8" fill="#fef08a" />
            </g>

            <!-- Blossom 6: Mid-left -->
            <g v-if="displayedBlooms >= 6" transform="translate(132, 126) scale(0.75)">
              <circle cx="0" cy="0" r="4.5" fill="url(#bloomGrad)" />
              <circle cx="-3" cy="-2" r="3" fill="url(#bloomGrad)" />
              <circle cx="3" cy="-2" r="3" fill="url(#bloomGrad)" />
              <circle cx="0" cy="0" r="1.5" fill="#fef08a" />
            </g>

            <!-- Blossom 7: Upper-right -->
            <g v-if="displayedBlooms >= 7" transform="translate(276, 110) scale(0.75)">
              <circle cx="0" cy="0" r="4.5" fill="url(#bloomGrad)" />
              <circle cx="-3" cy="-2" r="3" fill="url(#bloomGrad)" />
              <circle cx="3" cy="-2" r="3" fill="url(#bloomGrad)" />
              <circle cx="0" cy="0" r="1.5" fill="#fef08a" />
            </g>

            <!-- Blossom 8: Lower-left inner -->
            <g v-if="displayedBlooms >= 8" transform="translate(118, 226) scale(0.7)">
              <circle cx="0" cy="0" r="4" fill="url(#bloomGrad)" />
              <circle cx="0" cy="0" r="1.5" fill="#fef08a" />
            </g>

            <!-- Gentle fallen petals resting on the zen base -->
            <path d="M 172 268 C 170 266, 175 264, 178 266 C 180 268, 174 270, 172 268 Z" fill="#f472b6" opacity="0.85" />
            <path d="M 224 269 C 222 267, 228 265, 230 267 C 232 269, 226 271, 224 269 Z" fill="#f472b6" opacity="0.85" />
            <path d="M 198 271 C 196 269, 202 268, 204 270 C 206 272, 200 273, 198 271 Z" fill="#fbcfe8" opacity="0.9" />
          </g>
        </g>
      </svg>
    </div>

    <!-- Footer: Milestone Progress Bar & Hints -->
    <div class="mt-2 rounded-xl border border-border/80 bg-surface/60 p-3.5 backdrop-blur-sm">
      <div class="flex items-center justify-between text-xs font-medium">
        <span class="text-muted-foreground flex items-center gap-1.5">
          <Info class="h-3.5 w-3.5 text-primary" />
          {{ activeStage === 5 ? 'Mastery Sustained' : 'Next Growth Milestone' }}
        </span>
        <span class="font-mono text-foreground font-semibold">
          {{ Math.round(props.treeStage.progressPercentage) }}%
        </span>
      </div>

      <!-- Progress Track -->
      <div class="mt-2 h-2 w-full overflow-hidden rounded-full bg-surface border border-border/40">
        <div
          class="h-full rounded-full bg-gradient-to-r from-teal-500 to-emerald-400 transition-all duration-700 ease-out"
          :style="{ width: `${props.treeStage.progressPercentage}%` }"
        />
      </div>

      <p class="mt-2 text-[11px] text-muted-foreground leading-relaxed">
        {{ props.treeStage.nextMilestoneHint }}
      </p>
    </div>
  </div>
</template>
