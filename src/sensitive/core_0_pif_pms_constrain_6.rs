#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_6` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_6` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT` reader - core_0_pif_pms_constrain_world_1_bt"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT` writer - core_0_pif_pms_constrain_world_1_bt"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0` reader - core_0_pif_pms_constrain_world_1_i2c_ext0"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0` writer - core_0_pif_pms_constrain_world_1_i2c_ext0"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0` reader - core_0_pif_pms_constrain_world_1_uhci0"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0` writer - core_0_pif_pms_constrain_world_1_uhci0"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT` reader - core_0_pif_pms_constrain_world_1_rmt"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT` writer - core_0_pif_pms_constrain_world_1_rmt"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC` reader - core_0_pif_pms_constrain_world_1_ledc"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC` writer - core_0_pif_pms_constrain_world_1_ledc"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB` reader - core_0_pif_pms_constrain_world_1_bb"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB` writer - core_0_pif_pms_constrain_world_1_bb"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP` reader - core_0_pif_pms_constrain_world_1_timergroup"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP` writer - core_0_pif_pms_constrain_world_1_timergroup"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1` reader - core_0_pif_pms_constrain_world_1_timergroup1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1` writer - core_0_pif_pms_constrain_world_1_timergroup1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER` reader - core_0_pif_pms_constrain_world_1_systimer"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER` writer - core_0_pif_pms_constrain_world_1_systimer"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_1_bt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_1_i2c_ext0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_i2c_ext0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_1_uhci0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_uhci0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_1_rmt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_rmt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_1_ledc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_ledc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_1_bb"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bb(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_1_timergroup"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_timergroup(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_1_timergroup1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_timergroup1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_1_systimer"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_systimer(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_1_bt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bt(&mut self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W { w: self }
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_1_i2c_ext0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_i2c_ext0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W { w: self }
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_1_uhci0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_uhci0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W { w: self }
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_1_rmt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_rmt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W { w: self }
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_1_ledc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_ledc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W { w: self }
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_1_bb"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bb(&mut self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W { w: self }
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_1_timergroup"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_timergroup(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W { w: self }
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_1_timergroup1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_timergroup1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W { w: self }
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_1_systimer"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_systimer(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_6](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_6::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_6::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_6 to value 0xfcc3_0cf3"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcc3_0cf3
    }
}
