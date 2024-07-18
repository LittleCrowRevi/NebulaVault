using System.Collections;
using System.Collections.Generic;
using System.Numerics;
using TMPro;
using UnityEngine;
using Vector3 = UnityEngine.Vector3;

[RequireComponent( typeof( RectTransform ) )]
[RequireComponent( typeof( CanvasRenderer ) )]
[RequireComponent( typeof( TextMeshProUGUI ) )]
public class FloatingDamageText : FlexibleUi
{
    public Vector3 initialOffset, finaloffest;

    public float fadeDuration;

    private float fadeStartTime;

    // Start is called before the first frame update
    private void Start()
    {
        fadeStartTime = Time.time;
        initialOffset = transform.localPosition;
        finaloffest   = initialOffset + new Vector3( 1, 0, 0 );
    }

    // Update is called once per frame
    public override void Update()
    {
        base.Update();

        var text = GetComponent< TMP_Text >();
        if ( !text ) return;

        if ( skinData && skinData.font ) text.font = skinData.font;

        if ( !Application.isPlaying ) return;

        var progress = ( Time.time - fadeStartTime ) / fadeDuration;
        if ( progress <= 1 )
        {
            transform.localPosition = Vector3.Lerp( initialOffset, finaloffest, progress );
        }
        else Destroy( gameObject );
    }
}