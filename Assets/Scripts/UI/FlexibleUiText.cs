using System;
using System.Collections;
using System.Collections.Generic;
using System.Text;
using TMPro;
using UnityEngine;
using UnityEngine.Serialization;

[RequireComponent( typeof( RectTransform ) )]
[RequireComponent( typeof( CanvasRenderer ) )]
[RequireComponent( typeof( TextMeshProUGUI ) )]
public class FlexibleUiText : FlexibleUi
{
    [Header( "Type Data" )]
    public UnityEngine.Object observedEntity;

    [ConditionalProperty( "observedEntity", typeof( EntityData ) )]
    [Tooltip( "Stat Type to observe." )]
    public StatType observedStat;

    public override void Update()
    {
        base.Update();

        var text = GetComponent< TMP_Text >();
        if ( !text ) return;

        if ( skinData && skinData.font ) text.font = skinData.font;

        if ( !observedEntity ) return;
        switch ( observedEntity )
        {
            case Entity data:
                var dataString = new StringBuilder();
                switch ( observedStat )
                {
                    case StatType.Focus:
                        dataString.Append( data.Focus );
                        break;

                    case StatType.Mind:
                        dataString.Append( data.Mind );
                        break;

                    case StatType.Body:
                        dataString.Append( data.Body );
                        break;
                    
                    case StatType.Health:
                        dataString.Append( $"{data.CurrentHealth}/{data.Health}" );
                        break;
                }

                text.text = $"{dataString}";
                break;
        }
    }
}