#[doc = "Register `USB_INTR_MAP` reader"]
pub struct R(crate::R<USB_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_INTR_MAP` writer"]
pub struct W(crate::W<USB_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_INTR_MAP_SPEC>;
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
impl From<crate::W<USB_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_INTR_MAP` reader - reg_core0_usb_intr_map"]
pub struct USB_INTR_MAP_R(crate::FieldReader<u8, u8>);
impl USB_INTR_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_INTR_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_INTR_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_INTR_MAP` writer - reg_core0_usb_intr_map"]
pub struct USB_INTR_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INTR_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - reg_core0_usb_intr_map"]
    #[inline(always)]
    pub fn usb_intr_map(&self) -> USB_INTR_MAP_R {
        USB_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_usb_intr_map"]
    #[inline(always)]
    pub fn usb_intr_map(&mut self) -> USB_INTR_MAP_W {
        USB_INTR_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb intr map register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_intr_map](index.html) module"]
pub struct USB_INTR_MAP_SPEC;
impl crate::RegisterSpec for USB_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_intr_map::R](R) reader structure"]
impl crate::Readable for USB_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_intr_map::W](W) writer structure"]
impl crate::Writable for USB_INTR_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_INTR_MAP to value 0"]
impl crate::Resettable for USB_INTR_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
