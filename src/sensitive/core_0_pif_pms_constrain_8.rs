#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_8` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_8` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP` reader - core_0_pif_pms_constrain_world_1_usb_wrap"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP` writer - core_0_pif_pms_constrain_world_1_usb_wrap"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI` reader - core_0_pif_pms_constrain_world_1_crypto_peri"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI` writer - core_0_pif_pms_constrain_world_1_crypto_peri"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA` reader - core_0_pif_pms_constrain_world_1_crypto_dma"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA` writer - core_0_pif_pms_constrain_world_1_crypto_dma"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC` reader - core_0_pif_pms_constrain_world_1_apb_adc"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC` writer - core_0_pif_pms_constrain_world_1_apb_adc"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR` reader - core_0_pif_pms_constrain_world_1_bt_pwr"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR` writer - core_0_pif_pms_constrain_world_1_bt_pwr"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE` reader - core_0_pif_pms_constrain_world_1_usb_device"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE` writer - core_0_pif_pms_constrain_world_1_usb_device"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM` reader - core_0_pif_pms_constrain_world_1_system"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM` writer - core_0_pif_pms_constrain_world_1_system"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE` reader - core_0_pif_pms_constrain_world_1_sensitive"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE` writer - core_0_pif_pms_constrain_world_1_sensitive"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT` reader - core_0_pif_pms_constrain_world_1_interrupt"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT` writer - core_0_pif_pms_constrain_world_1_interrupt"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY` reader - core_0_pif_pms_constrain_world_1_dma_copy"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY` writer - core_0_pif_pms_constrain_world_1_dma_copy"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG` reader - core_0_pif_pms_constrain_world_1_cache_config"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG` writer - core_0_pif_pms_constrain_world_1_cache_config"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD` reader - core_0_pif_pms_constrain_world_1_ad"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD` writer - core_0_pif_pms_constrain_world_1_ad"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO` reader - core_0_pif_pms_constrain_world_1_dio"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO` writer - core_0_pif_pms_constrain_world_1_dio"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER` reader - core_0_pif_pms_constrain_world_1_world_controller"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER` writer - core_0_pif_pms_constrain_world_1_world_controller"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - core_0_pif_pms_constrain_world_1_usb_wrap"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_usb_wrap(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_1_crypto_peri"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_crypto_peri(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_1_crypto_dma"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_crypto_dma(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - core_0_pif_pms_constrain_world_1_apb_adc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_apb_adc(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - core_0_pif_pms_constrain_world_1_bt_pwr"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bt_pwr(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_1_usb_device"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_usb_device(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_1_system"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_system(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - core_0_pif_pms_constrain_world_1_sensitive"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_sensitive(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - core_0_pif_pms_constrain_world_1_interrupt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_interrupt(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_1_dma_copy"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_dma_copy(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - core_0_pif_pms_constrain_world_1_cache_config"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_cache_config(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_1_ad"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_ad(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_1_dio"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_dio(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_1_world_controller"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_world_controller(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - core_0_pif_pms_constrain_world_1_usb_wrap"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_usb_wrap(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_W { w: self }
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_1_crypto_peri"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_crypto_peri(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_W { w: self }
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_1_crypto_dma"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_crypto_dma(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_W { w: self }
    }
    #[doc = "Bits 8:9 - core_0_pif_pms_constrain_world_1_apb_adc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_apb_adc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_W { w: self }
    }
    #[doc = "Bits 12:13 - core_0_pif_pms_constrain_world_1_bt_pwr"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bt_pwr(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_W { w: self }
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_1_usb_device"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_usb_device(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_W { w: self }
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_1_system"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_system(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_W { w: self }
    }
    #[doc = "Bits 18:19 - core_0_pif_pms_constrain_world_1_sensitive"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_sensitive(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_W { w: self }
    }
    #[doc = "Bits 20:21 - core_0_pif_pms_constrain_world_1_interrupt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_interrupt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_W { w: self }
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_1_dma_copy"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_dma_copy(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_W { w: self }
    }
    #[doc = "Bits 24:25 - core_0_pif_pms_constrain_world_1_cache_config"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_cache_config(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_W { w: self }
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_1_ad"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_ad(&mut self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_W { w: self }
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_1_dio"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_dio(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_W { w: self }
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_1_world_controller"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_world_controller(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_8](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_8_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_8::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_8::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_8 to value 0xffff_f3fc"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_f3fc
    }
}
