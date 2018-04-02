#include <CoreFoundation/CoreFoundation.h>
#include <CoreServices/CoreServices.h>
#include <QuickLook/QuickLook.h>

#define BUNDLE_IDENTIFIER "de.ccl.webfun.qlgenerator"

extern OSStatus generate_thumbnail(const char * bundlePath, const char * gamePath, size_t* length, uint8 **buffer);

OSStatus GenerateThumbnailForURL(void *thisInterface, QLThumbnailRequestRef thumbnail, CFURLRef url, CFStringRef contentTypeUTI, CFDictionaryRef options, CGSize maxSize);
void CancelThumbnailGeneration(void *thisInterface, QLThumbnailRequestRef thumbnail);

/* -----------------------------------------------------------------------------
    Generate a thumbnail for file

   This function's job is to create thumbnail for designated file as fast as possible
   ----------------------------------------------------------------------------- */

OSStatus GenerateThumbnailForURL(void *thisInterface, QLThumbnailRequestRef thumbnail, CFURLRef url, CFStringRef contentTypeUTI, CFDictionaryRef options, CGSize maxSize)
{
    CFStringRef path = CFURLCopyFileSystemPath(url, kCFURLPOSIXPathStyle);
    CFBundleRef bundle = CFBundleGetBundleWithIdentifier(CFSTR(BUNDLE_IDENTIFIER));
    CFURLRef bundleURL = CFBundleCopyBundleURL(bundle);
    CFStringRef bundlePath = CFURLCopyFileSystemPath(bundleURL, kCFURLPOSIXPathStyle);
    
    const char * bundle_path = CFStringGetCStringPtr(bundlePath, kCFStringEncodingUTF8);
    const char * cpath = CFStringGetCStringPtr(path, kCFStringEncodingUTF8);
    
    size_t length = 0;
    uint8 *buffer = NULL;
    OSStatus result = generate_thumbnail(bundle_path, cpath, &length, &buffer);
    
    CFDataRef imageData = CFDataCreateWithBytesNoCopy(NULL, buffer, length, NULL);
    QLThumbnailRequestSetImageWithData(thumbnail, imageData, NULL);
    CFRelease(imageData);
    
    CFRelease(bundlePath);
    CFRelease(bundleURL);
    CFRelease(path);

    return result;
}

void CancelThumbnailGeneration(void *thisInterface, QLThumbnailRequestRef thumbnail)
{
    // Implement only if supported
}
