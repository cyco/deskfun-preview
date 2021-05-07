#include <CoreFoundation/CoreFoundation.h>
#include <CoreServices/CoreServices.h>
#include <QuickLook/QuickLook.h>

#define BUNDLE_IDENTIFIER "de.ccl.deskfun-preview.qlgenerator"

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
    if (bundle == NULL) {
        return -1;
    }
    
    CFURLRef bundleURL = CFBundleCopyBundleURL(bundle);
    CFStringRef bundlePath = CFURLCopyFileSystemPath(bundleURL, kCFURLPOSIXPathStyle);
    
    const char * bundle_path = CFStringGetCStringPtr(bundlePath, kCFStringEncodingUTF8);
    const char * cpath = CFStringGetCStringPtr(path, kCFStringEncodingUTF8);
    
    size_t length = 0;
    uint8 *buffer = NULL;
    OSStatus result = generate_thumbnail(bundle_path, cpath, &length, &buffer);
    
    const void* keys[1] = { kCGImageSourceTypeIdentifierHint };
    CFTypeRef values[1] = { kUTTypePNG };
    CFDictionaryRef properties = CFDictionaryCreate(NULL, keys, values, 1, NULL, NULL);
    
    CFDataRef imageData = CFDataCreateWithBytesNoCopy(NULL, buffer, length, kCFAllocatorNull);
    QLThumbnailRequestSetImageWithData(thumbnail, imageData, properties);
    CFRelease(imageData);
    
    CFRelease(properties);
    
    CFRelease(bundlePath);
    CFRelease(bundleURL);
    CFRelease(path);

    return result;
}

void CancelThumbnailGeneration(void *thisInterface, QLThumbnailRequestRef thumbnail)
{
    // Implement only if supported
}
