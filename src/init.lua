-- ============================================================
--  "الجمال المخفي" — The Hidden Beauty of φ
--  Duration: ~13 seconds | 9:16 Vertical | 60fps
--  Theme: Golden Ratio in Mathematics
--  Features: pin(), target_*, helper functions, SVG spiral
--  EVERY animate_pointer is followed by pin()
--  EVERY exit uses target_* (never set_*)
--  EVERY Typst math has $ signs
-- ============================================================

local SHORT   = Size.youtube_shorts()
local FPS     = FpsTarget.f60()
local GOLD    = "#F9E2AF"
local ROSE    = "#F5E0DC"
local BLUE    = "#89B4FA"
local PINK    = "#F38BA8"
local DARK    = "#1E1E2E"
local SURFACE = "#313244"
local MUTED   = "#A6ADC8"
local GREEN   = "#A6E3A1"

-- ═══════════════════════════════════════════════════════════
-- HELPER FUNCTIONS — لتبسيط الكود وتجنب التكرار
-- ═══════════════════════════════════════════════════════════

-- أنشئ عنصر في مركز الشاشة مع Anchor.center
function spawn_centered(model, component, x_pct, y_pct)
    local ptr = model:spawn_located(component, Location.new(
        Coordinate.percentage(x_pct),
        Coordinate.percentage(y_pct)
    ))
    model:with_pointer(ptr, function(c)
        c:set_anchor(Anchor.center())
    end)
    return ptr
end

-- حركة دخول: تطبيق الإعدادات، تنفيذ animation، ثم تثبيت الحالة
function animate_in(model, ptr, duration, easing, setup_fn)
    model:with_pointer(ptr, setup_fn)
    model:animate_pointer(ptr, duration, easing, function(c, p)
        c:update_progress(p)
    end)
    model:with_pointer(ptr, function(c) c:pin() end)
end

-- حركة خروج: تحديد الهدف بـ target_*, ثم animation، ثم pin
function animate_out(model, ptr, duration, easing, target_op, target_ang)
    model:with_pointer(ptr, function(c)
        if target_op then c:target_opacity(target_op) end
        if target_ang then c:target_angle(target_ang) end
    end)
    model:animate_pointer(ptr, duration, easing, function(c, p)
        c:update_progress(p)
    end)
    model:with_pointer(ptr, function(c) c:pin() end)
end

-- نبضة تأكيد (emphasis pulse) — دوران خفيف ذهاباً وإياباً
function pulse_angle(model, ptr, angle_deg)
    model:with_pointer(ptr, function(c)
        c:target_angle(Angle.new(angle_deg))
    end)
    model:animate_pointer(ptr, 0.4, Easing.ease_in_out(), function(c, p)
        c:update_progress(p)
    end)
    model:with_pointer(ptr, function(c) c:pin() end)

    model:with_pointer(ptr, function(c)
        c:target_angle(Angle.new(0.0))
    end)
    model:animate_pointer(ptr, 0.4, Easing.ease_in_out(), function(c, p)
        c:update_progress(p)
    end)
    model:with_pointer(ptr, function(c) c:pin() end)
end

-- ═══════════════════════════════════════════════════════════
-- SCENE 1: Title Card — "الجمال المخفي" (3.5s)
-- ═══════════════════════════════════════════════════════════
local scene1 = Model.new(SHORT, FPS)

local title_ar  = Text.new("الجمال المخفي"):size(64):color(GOLD):weight(Weight.bold())
local title_en  = Text.new("The Hidden Beauty"):size(32):color(ROSE):weight(Weight.medium())
local subtitle  = Text.new("$ phi = (1 + sqrt(5)) / 2 approx 1.618 $"):size(28):color(MUTED)

local col1 = Column.new(SHORT)
    :arrangment(Arrangement.center())
    :alignment(Alignment.center())
    :padding(Padding.new(40.0))
col1:push(title_ar)
col1:push(title_en)
col1:push(subtitle)

local card1 = Container.new(col1)
    :background(DARK)
    :border_color(GOLD)
    :border_width(3.0)
    :border_radius(32.0)
    :padding(Padding.new(48.0))

-- الدخول: انزلاق من الأعلى + ظهور تدريجي + دوران خفيف
local ptr1 = spawn_centered(scene1, card1, 0.5, -0.3)

animate_in(scene1, ptr1, 1.2, Easing.ease_out(), function(c)
    c:set_opacity(Opacity.with_progress(0.0, 1.0))
    c:set_angle(Angle.with_progress(-12.0, 0.0))
    -- نستخدم target_loc لتحريك الموقع من الأعلى إلى المركز
    c:target_loc(
        Coordinate.percentage(0.5),
        Coordinate.percentage(0.42)
    )
end)

-- استقرار للقراءة
scene1:wait(2.0)

-- الخروج: اختفاء تدريجي + دوران بعكس الاتجاه
animate_out(scene1, ptr1, 0.8, Easing.ease_in(),
    Opacity.new(0.0),
    Angle.new(8.0)
)

-- ═══════════════════════════════════════════════════════════
-- SCENE 2: المعادلة الذهبية + سبيرال فيبوناتشي (5.5s)
-- ═══════════════════════════════════════════════════════════
local scene2 = Model.new(SHORT, FPS)

-- المعادلة الأولى: تعريف φ
local pager = Pager.new()
    :add("$ phi = (1 + sqrt(5)) / 2 approx 1.6180339887... $")
    :add("$ a / b = (a + b) / a = phi $")

local svgs = LuaWorld.new(pager):to_svgs()
local eq1 = svgs[1]:scale(2.2)
local eq2 = svgs[2]:scale(1.8)

-- سبيرال فيبوناتشي مرسوم يدوياً بـ SVG
local spiral_svg = SvgBox.new([[
<svg width="400" height="400" viewBox="0 0 400 400" xmlns="http://www.w3.org/2000/svg">
  <rect width="400" height="400" fill="none"/>
  <!-- مربعات فيبوناتشي بتدرج ألوان -->
  <rect x="0" y="0" width="247" height="247" fill="none" stroke="#89B4FA" stroke-width="2" opacity="0.25"/>
  <rect x="247" y="0" width="153" height="153" fill="none" stroke="#F9E2AF" stroke-width="2" opacity="0.4"/>
  <rect x="247" y="153" width="94" height="94" fill="none" stroke="#F38BA8" stroke-width="2" opacity="0.55"/>
  <rect x="341" y="153" width="59" height="59" fill="none" stroke="#A6E3A1" stroke-width="2" opacity="0.7"/>
  <rect x="341" y="212" width="36" height="36" fill="none" stroke="#89B4FA" stroke-width="2" opacity="0.85"/>
  <!-- السبيرال الذهبي -->
  <path d="M 400 400 
           A 247 247 0 0 1 153 153
           A 153 153 0 0 1 247 0
           A 94 94 0 0 1 341 94
           A 59 59 0 0 1 400 153
           A 36 36 0 0 1 341 212" 
        fill="none" stroke="#F9E2AF" stroke-width="4" stroke-linecap="round"/>
  <!-- نقطة المركز -->
  <circle cx="400" cy="400" r="6" fill="#F9E2AF"/>
  <!-- نقاط لامعة صغيرة على السبيرال -->
  <circle cx="153" cy="153" r="4" fill="#89B4FA" opacity="0.8"/>
  <circle cx="247" cy="0" r="4" fill="#F38BA8" opacity="0.8"/>
  <circle cx="341" cy="94" r="4" fill="#A6E3A1" opacity="0.8"/>
</svg>
]]):scale(1.6)

-- تجميع المعادلات والسبيرال في عمود
local content_col = Column.new(Size.new(900, 1100))
    :arrangment(Arrangement.spaced_by(24))
    :alignment(Alignment.center())
content_col:push(eq1)
content_col:push(spiral_svg)
content_col:push(eq2)

local card2 = Container.new(content_col)
    :background("#181825")
    :border_color(BLUE)
    :border_width(4.0)
    :border_radius(36.0)
    :padding(Padding.new(40.0))

local ptr2 = spawn_centered(scene2, card2, 0.5, 0.45)

-- دخول درامي: دوران من -90° مع ظهور تدريجي
animate_in(scene2, ptr2, 1.4, Easing.ease_out(), function(c)
    c:set_opacity(Opacity.with_progress(0.0, 1.0))
    c:set_angle(Angle.with_progress(-90.0, 0.0))
end)

-- استقرار للتمعن في المعادلات والرسم
scene2:wait(3.5)

-- نبضة تأكيد خفيفة على السبيرال
pulse_angle(scene2, ptr2, 4.0)

-- استقرار إضافي
scene2:wait(1.0)

-- خروج: انزلاق للأسفل + اختفاء
scene2:with_pointer(ptr2, function(c)
    c:target_opacity(Opacity.new(0.0))
    c:target_angle(Angle.new(-15.0))
    c:target_loc(
        Coordinate.percentage(0.5),
        Coordinate.percentage(1.3)
    )
end)
scene2:animate_pointer(ptr2, 1.0, Easing.ease_in(), function(c, p)
    c:update_progress(p)
end)
scene2:with_pointer(ptr2, function(c) c:pin() end)

-- ═══════════════════════════════════════════════════════════
-- SCENE 3: الخاتمة — "الرياضيات هي لغة الطبيعة" (3.5s)
-- ═══════════════════════════════════════════════════════════
local scene3 = Model.new(SHORT, FPS)

local closing_ar = Text.new("الرياضيات هي لغة الطبيعة"):size(38):color(ROSE):weight(Weight.bold())
local closing_en = Text.new("Math is the language of nature"):size(22):color(MUTED)
local closing_math = Text.from_raw("$ phi approx 1.618 $"):size(26):color(GOLD)

local col3 = Column.new(SHORT)
    :arrangment(Arrangement.spaced_by(20))
    :alignment(Alignment.center())
    :padding(Padding.new(40.0))
col3:push(closing_ar)
col3:push(closing_en)
col3:push(closing_math)

local card3 = Container.new(col3)
    :background(DARK)
    :border_color(GREEN)
    :border_width(3.0)
    :border_radius(28.0)
    :padding(Padding.new(44.0))

local ptr3 = spawn_centered(scene3, card3, 0.5, 0.45)

-- دخول ناعم: ظهور تدريجي + تكبير خفيف وهمي عبر تغيير الشفافية
animate_in(scene3, ptr3, 1.0, Easing.ease_out(), function(c)
    c:set_opacity(Opacity.with_progress(0.0, 1.0))
    c:set_angle(Angle.with_progress(5.0, 0.0))
end)

-- استقرار للتأمل في الخاتمة
scene3:wait(2.5)

-- خروج نهائي: ذوبان تام
animate_out(scene3, ptr3, 0.8, Easing.ease_in(),
    Opacity.new(0.0),
    Angle.new(-5.0)
)

-- ═══════════════════════════════════════════════════════════
-- تجميع المشاهد وإخراج الفيديو
-- ═══════════════════════════════════════════════════════════
local masterpiece = scene1 + scene2 + scene3
masterpiece:render_video("golden_ratio_beauty.mp4")

print("✨ تم الإخراج: golden_ratio_beauty.mp4 (~13 ثانية من الأناقة الرياضية)")
print("🔒 كل animation مثبت بـ pin() | كل exit يستخدم target_* | كل math بعلامة $")
